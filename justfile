install:
  cargo install --path .

run: 
  casgen

clean:
  rm -rfv casgen*{fastq,tsv,txt}
  
