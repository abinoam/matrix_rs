# Require Matrix here so it will not be overwritten when
# required again from within the Matrix gem test suite
require "matrix"

# Provide a substitute for Test::Unit::TestCase
# So the matrix gem tests runs without any modification

module Test
  module Unit
    TestCase = Minitest::Test

    class TestCase
      # Matrix has to be overriden inside TestCase
      # or otherwise will not be seen by the tests
      # from the Matrix gem test suite.
      Matrix = MatrixRs

      # Alias some matrix gem test suite method names
      # to their minitest counterparts
      alias assert_raise     assert_raises
      alias assert_not_equal refute_equal
      alias assert_not_same  refute_same

      # Intercept assert_equal to suppress messages about
      # deprecating assert_equal with nil as argument
      alias _assert_equal assert_equal

      def assert_equal(first, second, msg=nil)
        return assert_nil(first, msg) if second.nil?
        return assert_nil(second, msg) if first.nil?
        _assert_equal first, second, msg
      end

      # Provide missing assert_block
      def assert_block
        assert yield
      end
    end
  end
end