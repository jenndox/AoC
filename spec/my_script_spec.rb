RSpec.describe 'my_script' do
  it 'prints hello from our script' do
    expect(STDOUT).to receive(:puts).with('hello from our script!')
    require_relative '../bin/my_script'
  end
end
