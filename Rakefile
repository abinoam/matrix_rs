# frozen_string_literal: true

require "bundler/gem_tasks"
require "rake/testtask"

Rake::TestTask.new(:test) do |t|
  t.libs << "test"
  t.libs << "lib"
  t.test_files = FileList["test/**/*_test.rb"]
end

task default: :test

# Thermite
require 'thermite/tasks'

Thermite::Tasks.new

desc 'Run Rust & Ruby testsuites'
task test: ['thermite:build', 'thermite:test'] do
  # …
end