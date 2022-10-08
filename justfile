install:
  cargo install --path .

run: 
  casgen

clean:
  rm -rfv casgen*{fastq,fasta,tsv,txt}
  
