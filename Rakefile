# For general help with Ruby packages, see
# https://w.amazon.com/index.php/Ruby/QuickStart
#
# For help with Brazil Rakefiles, see
# https://w.amazon.com/index.php/BrazilRake

require 'brazil'
require 'brazil/librarypackagetask'
require 'brazil/copysrctask'
require 'brazil/rubysyntaxtask'
require 'rspec/core/rake_task'
require 'rubocop/rake_task'

desc 'Copy libraries from lib/'
Brazil::LibraryPackageTask.new(:lib)
task :all => :lib

desc 'Copy scripts from bin/'
Brazil::CopySrcTask.new(:bin) do |t|
  t.excludes << 'README.md'
end
task :all => :bin

# You may remove this task if you don't want syntax checking during build
desc 'Syntax-check bin/ and lib/'
Brazil::RubySyntaxTask.new do |task|
  task.files
      .include('lib/**/*.rb')
      .include('bin/**/*')
      .exclude('bin/README.md')
end
task :all => :syntax

# Ruby code style checking/enforcement via Rubocop
RuboCop::RakeTask.new(:rubocop) do |task|
  task.patterns = %w[bin lib spec]
  task.formatters = %w[fuubar html]
  task.options += [
    '--display-style-guide',
    '--display-cop-names',
    '--extra-details',
    '--out', File.join(Brazil.documentation_dir, 'rubocop.html')
  ]
  task.fail_on_error = true
end
task :release => :rubocop

desc 'Run our unit tests in spec/ with coverage'
RSpec::Core::RakeTask.new
task :release => :spec

# Uncomment the following line to make 'release' the default task if no other
# arguments are provided to brazil-build.
# task :default => :release
