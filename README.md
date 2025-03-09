# TODO:
get mic input into byte array
split data into small sections (maybe 100ms?? can't be too small)
- might have to use 'hamming window function' to avoid 'spectral leakage'
fast fourier transform data
create spectrogram (maybe make a quick visualizer for this?)
fingerprint data (decide on algorithm)
get mp3 files into byte array
store mp3 data (for searchable songs)
hash lookup for song matches
decide which song match is the best match (probably smthing like most clustered matches)
make frontend
