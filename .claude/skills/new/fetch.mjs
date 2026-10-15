// Usage: node fetch.mjs <problem-id | title-slug>
// Prints the problem's metadata, statement (HTML), examples and starter code as JSON.
// Topic tags are left out on purpose: they would spoil the pattern.

const ENDPOINT = "https://leetcode.com/graphql";

async function graphql(query, variables) {
  const res = await fetch(ENDPOINT, {
    method: "POST",
    headers: { "Content-Type": "application/json", Referer: "https://leetcode.com" },
    body: JSON.stringify({ query, variables }),
  });
  if (!res.ok) throw new Error(`LeetCode responded ${res.status}`);
  const json = await res.json();
  if (json.errors) throw new Error(JSON.stringify(json.errors));
  return json.data;
}

async function resolveSlug(arg) {
  if (!/^\d+$/.test(arg)) return arg;
  const data = await graphql(
    `query($k: String) {
      questionList(categorySlug: "", limit: 50, skip: 0, filters: { searchKeywords: $k }) {
        data { questionFrontendId titleSlug }
      }
    }`,
    { k: arg },
  );
  const match = data.questionList.data.find((q) => q.questionFrontendId === arg);
  if (!match) throw new Error(`No problem with id ${arg}`);
  return match.titleSlug;
}

const arg = process.argv[2];
if (!arg) {
  console.error("Usage: node fetch.mjs <problem-id | title-slug>");
  process.exit(1);
}

try {
  const slug = await resolveSlug(arg);
  const { question } = await graphql(
    `query($s: String!) {
      question(titleSlug: $s) {
        questionFrontendId title titleSlug difficulty isPaidOnly
        content exampleTestcaseList
        codeSnippets { langSlug code }
      }
    }`,
    { s: slug },
  );
  if (!question) throw new Error(`No problem with slug ${slug}`);
  console.log(
    JSON.stringify(
      {
        id: question.questionFrontendId,
        title: question.title,
        url: `https://leetcode.com/problems/${question.titleSlug}/`,
        difficulty: question.difficulty,
        paidOnly: question.isPaidOnly,
        content: question.content,
        examples: question.exampleTestcaseList,
        snippets: Object.fromEntries((question.codeSnippets ?? []).map((s) => [s.langSlug, s.code])),
      },
      null,
      2,
    ),
  );
} catch (err) {
  console.error(`fetch failed: ${err.message}`);
  process.exitCode = 1;
}
