[![Build Status](https://travis-ci.com/abinoam/matrix_rs.svg?token=jLesqB3VvKRLAuZkesEy&branch=master)](https://travis-ci.com/abinoam/matrix_rs)

# MatrixRs

A Rust implementation of Ruby Matrix class.

I am studying Rust and I'll try to make a drop in replacement for Matrix Ruby library.

I'm fresh new to Rust so I don't know if I would be able to acomplish this.

## Roadmap

Done:
1. Add Matrix gem repository as git submodule
    - This will easy the task of keeping track with future updates
2. Run Matrix gem original test suite against MatrixRs class
3. Add rutie gem and boilerplate code
4. Add a fake `MatrixRs.[]` and conect Ruby to Rust
5. Be able to run tests that talk to the Rust fake implementation
6. Implemented `MatrixRs.[]`
7. Implemented simple `MatrixRs.to_s` over Debug trait
8. Implemented `MatrixRs.*` (dot product) over ndarray dot

OBS:
- Meanwhile MatrixRs is connected to its Rust counterpart.
- Some Rust code were added but it just doesn't do anything!

## Installation

Add this line to your application's Gemfile:

```ruby
gem 'matrix_rs'
```

And then execute:

    $ bundle install

Or install it yourself as:

    $ gem install matrix_rs

## Usage

TODO: Write usage instructions here

## Development

After checking out the repo, run `bin/setup` to install dependencies. Then, run `rake spec` to run the tests. You can also run `bin/console` for an interactive prompt that will allow you to experiment.

To install this gem onto your local machine, run `bundle exec rake install`. To release a new version, update the version number in `version.rb`, and then run `bundle exec rake release`, which will create a git tag for the version, push git commits and tags, and push the `.gem` file to [rubygems.org](https://rubygems.org).

## Contributing

Bug reports and pull requests are welcome on GitHub at https://github.com/[USERNAME]/matrix_rs. This project is intended to be a safe, welcoming space for collaboration, and contributors are expected to adhere to the [code of conduct](https://github.com/[USERNAME]/matrix_rs/blob/master/CODE_OF_CONDUCT.md).


## License

The gem is available as open source under the terms of the [MIT License](https://opensource.org/licenses/MIT).

## Code of Conduct

Everyone interacting in the MatrixRs project's codebases, issue trackers, chat rooms and mailing lists is expected to follow the [code of conduct](https://github.com/[USERNAME]/matrix_rs/blob/master/CODE_OF_CONDUCT.md).
