# infracloud-short-url

# install curl 
sudo apt install curl

# install Rust
Type the following command and follow the on screen instructions:<br />
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# program structure 

|-- README.md

`-- short-url

    |-- Cargo.lock

    |-- Cargo.toml

    |-- Dockerfile

    |-- run.sh

    |-- src

    |   |-- configuration_parameters.rs

    |   |-- generate_url_code

    |   |   `-- mod.rs

    |   |-- main.rs

    |   |-- reader

    |   |   `-- mod.rs

    |   |-- routes.rs

    |   `-- writer

    |       `-- mod.rs

    `-- test-bed

        `-- url_map.txt



# step to run the program

step 1. Give chmod 777 permission to run.sh script.<br />
step 2. Create 'test-bed' folder parallel to 'src' and create an empty file 'url_map.txt'.<br />
step 3. Execute the script using ./run.sh and start the services.<br />
step 4. Call the API from postman and get the response.<br />

# end points used 

1. /get_short_url/  

    accepts json containing url as body parameter

    Purpose : generates shortned url for given link

    Ex. {<br />
            "url":"https://www.google.com"<br />
        }
    
    Useage. GET         : http://127.0.0.1:8000/get_short_url/<br />
            json body   : {<br />
                              "url":"https://www.google.com"<br />
                          }
    
    Response. {<br />
                  "long_url": "https://www.google.com",<br />
                  "short_url": "127.0.0.1:8000/bSVhXsLu"<br />
              }

2. /{url_code}

    accepts url_code as path parameter

    Purpose : fetches original link from passed short url

    Ex. 127.0.0.1:8000/bSVhXsLu

    Useage. GET     : 127.0.0.1:8000/bSVhXsLu

    Response. {<br />
                  "long_url": "https://www.google.com",<br />
                  "short_url": "127.0.0.1:8000/bSVhXsLu"<br />
              }

# libraries used

nanoid      : used for generating random alpharumric code of specified size<br />
clap        : used for parsing arguments to the rut program<br />
actix-web   : used for firing the API<br />
serde       : used for Serializing and Deserializing structs<br />

