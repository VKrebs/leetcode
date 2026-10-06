namespace LeetCode.Problems.P0070ClimbingStairs
{
    public class Solution
    {
        // Space Optimization (Rolling Window)
        public int ClimbStairs(int n)
        {
            checked
            {
                int waysTwoBelow = 1;
                int waysOneBelow = 1;

                for (int i = 2; i <= n; i++)
                {
                    (waysTwoBelow, waysOneBelow) = (waysOneBelow, waysOneBelow + waysTwoBelow);
                }

                return waysOneBelow;
            }
        }

        // Tabulation
        public int ClimbStairs_Tabulation(int n)
        {
            checked
            {
                int[] table = new int[n + 1];
                table[0] = 1;
                table[1] = 1;

                for (int i = 2; i <= n; i++)
                {

                    table[i] = table[i - 1] + table[i - 2];
                }

                return table[n];
            }
        }

        // Memoization
        public int ClimbStairs_Memoization(int n)
        {
            return ClimbStairs_Memoization(n, []);
        }

        private int ClimbStairs_Memoization(int n, Dictionary<int, int> memo)
        {
            checked
            {
                if (n == 1) return 1;
                if (n == 2) return 2;
                if (memo.TryGetValue(n, out int result)) return result;

                checked
                {
                    result = ClimbStairs_Memoization(n - 1, memo) + ClimbStairs_Memoization(n - 2, memo);
                }
                memo.Add(n, result);
                return result;
            }
        }

        // Naive recursion
        public int ClimbStairs_NaiveRecursion(int n)
        {
            checked
            {
                if (n == 1) return 1;
                if (n == 2) return 2;
                return ClimbStairs_NaiveRecursion(n - 1) + ClimbStairs_NaiveRecursion(n - 2);
            }
        }
    }

    public class SolutionTests
    {
        [Theory]
        [InlineData(1, 1)]
        [InlineData(2, 2)]
        [InlineData(3, 3)]
        [InlineData(4, 5)]
        [InlineData(5, 8)]
        [InlineData(10, 89)]
        [InlineData(45, 1836311903)]
        public void ClimbStairs_ComputesTheCountOfWayToClimbStairs(int countOfStairs, int expectedCountOfWaysToClimb)
        {
            Assert.Equal(expectedCountOfWaysToClimb, new Solution().ClimbStairs(countOfStairs));
        }

        [Fact]
        public void ClimbStairs_WithTooManyPossibleWaysToClimbStairs_ThrowsOverflowException()
        {
            Assert.Throws<OverflowException>(() => new Solution().ClimbStairs(46));
        }

        [Theory]
        [InlineData(1, 1)]
        [InlineData(2, 2)]
        [InlineData(3, 3)]
        [InlineData(4, 5)]
        [InlineData(5, 8)]
        [InlineData(10, 89)]
        public void ClimbStairs_NaiveRecursion_ComputesTheCountOfWayToClimbStairs(int countOfStairs, int expectedCountOfWaysToClimb)
        {
            Assert.Equal(expectedCountOfWaysToClimb, new Solution().ClimbStairs_NaiveRecursion(countOfStairs));
        }

        [Theory]
        [InlineData(1, 1)]
        [InlineData(2, 2)]
        [InlineData(3, 3)]
        [InlineData(4, 5)]
        [InlineData(5, 8)]
        [InlineData(10, 89)]
        [InlineData(45, 1836311903)]
        public void ClimbStairs_Memoization_ComputesTheCountOfWayToClimbStairs(int countOfStairs, int expectedCountOfWaysToClimb)
        {
            Assert.Equal(expectedCountOfWaysToClimb, new Solution().ClimbStairs_Memoization(countOfStairs));
        }

        [Fact]
        public void ClimbStairs_Memoization_WithTooManyPossibleWaysToClimbStairs_ThrowsOverflowException()
        {
            Assert.Throws<OverflowException>(() => new Solution().ClimbStairs_Memoization(46));
        }

        [Theory]
        [InlineData(1, 1)]
        [InlineData(2, 2)]
        [InlineData(3, 3)]
        [InlineData(4, 5)]
        [InlineData(5, 8)]
        [InlineData(10, 89)]
        [InlineData(45, 1836311903)]
        public void ClimbStairs_Tabulation_ComputesTheCountOfWayToClimbStairs(int countOfStairs, int expectedCountOfWaysToClimb)
        {
            Assert.Equal(expectedCountOfWaysToClimb, new Solution().ClimbStairs_Tabulation(countOfStairs));
        }

        [Fact]
        public void ClimbStairs_Tabulation_WithTooManyPossibleWaysToClimbStairs_ThrowsOverflowException()
        {
            Assert.Throws<OverflowException>(() => new Solution().ClimbStairs_Tabulation(46));
        }
    }
}
