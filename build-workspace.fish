find ./ -type f -name Cargo.toml ! -path "./Cargo.toml" -printf '%h\n' |
                                                            sort -u |
                                                            awk '
                                                        BEGIN {print "[workspace]\nmembers = ["}
                                                        {lines[NR]=$0}
                                                        END {
                                                          for(i=1; i<=NR; i++) {
                                                            comma = (i<NR ? "," : "")
                                                            printf "  \"%s\"%s\n", lines[i], comma
                                                          }
                                                          print "]\nresolver = \"2\""
                                                        }
                                                        ' > Cargo.toml

