SimpleCov.start do
  # Set the coverage directory
  coverage_dir File.join('build', 'brazil-documentation', 'coverage')

  # Don't add coverage on actual spec files
  add_filter '/spec/'

  # Hook into SimpleCov after we're done with tests
  at_exit do
    begin
      # Remove our assets folder if it exists (causes issues)
      FileUtils.remove_dir(File.join(coverage_dir, 'assets'), force: true)
      # Output our coverage report
      SimpleCov.result.format!
    rescue => e
      puts "Could not generate SimpleCov metrics #{e}"
    end

    # Open the file and write to it
    coverage_data = File.join('build', 'generated-make', 'coverage-data.txt')
    File.open(coverage_data, 'w+') do |f|
      f << "ruby:line:#{SimpleCov.result.covered_percent}"
    end
  end
end
