require 'my_library'

RSpec.describe MyLibrary do
  describe '#hello' do
    it 'prints hello from our library' do
      expect(STDOUT).to receive(:puts).with('hello from our library!')
      subject.hello
    end
  end
end
