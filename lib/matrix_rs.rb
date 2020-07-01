# frozen_string_literal: true

require "matrix_rs/version"
require "rutie"

# MatrixRs is a Rust extension replacement
#   for Ruby standard library Matrix
#
class MatrixRs
  Rutie.new(:matrix_rs).init "Init_matrix_rs", __dir__
end
