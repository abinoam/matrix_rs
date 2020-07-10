# frozen_string_literal: true

require "test_helper"

class MatrixRsTest < Minitest::Test
  def test_that_it_has_a_version_number
    refute_nil ::MatrixRs::VERSION
  end

  # Temporariy returning Arrays
  def test_matrix_rs_empty_2_0
    assert_equal ::MatrixRs.empty(2, 0), [[], []]
    assert_equal ::MatrixRs.empty(2, 3), [[0, 0, 0], [0, 0, 0]]
    assert_equal ::MatrixRs.empty(3, 2), [[0, 0], [0, 0], [0, 0]]
  end

  def test_matrix_rs_brackets_doesnt_raises
    assert ::MatrixRs[[1, 2, 3], [4, 5, 6]]
  end

  def test_matrix_rs_brackets_returns_a_matrix_rs_instance
    assert_equal ::MatrixRs[[1, 2, 3], [4, 5, 6]].class, MatrixRs
  end
end
