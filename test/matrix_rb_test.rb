# frozen_string_literal: true

require "test_helper"

if ENV['RUN_RUBY_MATRIX_TEST_SUITE'] == 'true'
  # Run the Matrix gem test suite against MatrixRs
  require_relative "../vendor/matrix/test/matrix/test_matrix.rb"
end
