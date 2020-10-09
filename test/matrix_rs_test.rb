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
    assert ::MatrixRs[[1.2, 2.4, 3.6], [4.8, 5.1, 6.1]]
  end

  def test_matrix_rs_brackets_returns_a_matrix_rs_instance
    assert_equal ::MatrixRs[[1.2, 2.4, 3.6], [4.8, 5.1, 6.1]].class, MatrixRs
  end

  def test_matrix_rs_to_s
    assert_equal ::MatrixRs[[1.2, 2.4, 3.6], [4.8, 5.1, 6.1]].to_s,
                 "[[1.2, 2.4, 3.6],\n [4.8, 5.1, 6.1]]"
  end

  def test_matrix_dot
    mr     = MatrixRs[[1.1, 2.2, 3.3], [1.1, 2.2, 3.3]]
    nr     = MatrixRs[[1.1, 2.2], [3.3, 4.4], [5.5, 6.6]]
    expect = "[[26.62, 33.88],\n [26.62, 33.88]]"
    assert_equal (mr * nr).to_s, expect
  end

  def test_matrix_dot_with_integers
    mr     = MatrixRs[[1, 2, 3], [1, 2, 3]]
    nr     = MatrixRs[[1, 2], [3, 4], [5, 6]]
    expect = "[[22, 28],\n [22, 28]]"
    assert_equal (mr * nr).to_s, expect
  end

  def test_matrix_fetch
    mr = MatrixRs[[1, 2, 3], [4, 5, 6]]
    assert_equal mr[0, 0], 1
    assert_equal mr[0, 1], 2
    assert_equal mr[1, 0], 4
    assert_equal mr[1, 2], 6
  end
end
