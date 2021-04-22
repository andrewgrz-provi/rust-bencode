RSpec.describe RutieRubyExample do
  it "has a version number" do
    expect(RutieRubyExample::VERSION).not_to be nil
  end

  it "parses bencode ints" do
    expect(BenCode.parse("i10e")).to eq(10)
    expect(BenCode.parse("le")).to eq([])
    expect(BenCode.parse("li10ee")).to eq([10])
    expect(BenCode.parse("li10ei15ee")).to eq([10, 15])
    expect(BenCode.parse("lleli8eee")).to eq([[], [8]])
  end
end
