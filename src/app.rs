/*
 * Copyright 2022 XXIV
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
use std::io::Cursor;
use reqwest::blocking::Response;
use toonify::{Toonify, ToonifyError, ToonifyFile};

pub fn app(args: Vec<String>) {
    if args.len() == 1 {
        print!("Usage: toonify -FILE \"API-KEY\" \"path-to-image\" [-options]\n");
        print!("           (Photo of any face into a cartoon from given image)\n");
        print!("   or  toonify -URI \"API-KEY\" \"image-URI\" [-options]\n");
        print!("           (Photo of any face into a cartoon from given image URI)\n");
        print!("where options include:\n");
        print!("    --save \"path\" \"image-name\"          save image")
    } else {
        match args.get(1) {
            Some(arg_one) => if arg_one.eq("-FILE") {
                match args.get(2) {
                    Some(arg_two) => {
                        match args.get(3) {
                            Some(arg_three) => {
                                match args.get(4) {
                                    Some(arg_four) => if arg_four.eq("--save") {
                                        match args.get(5) {
                                            Some(arg_five) => {
                                                match args.get(6) {
                                                    Some(arg_six) => {
                                                        let toon = ToonifyFile::new(arg_three,arg_two);
                                                        match toon.image() {
                                                            Ok(image) => {
                                                                match save_image(image.as_str(),arg_five,
                                                                                 arg_six) {
                                                                    Ok(_) => print!("Image saved successfully."),
                                                                    Err(err) => if err.eq("null") {
                                                                        print!("Error: Something went wrong")
                                                                    } else {
                                                                        print!("Error: {}", err)
                                                                    }
                                                                }
                                                            },
                                                            Err(err) => match err {
                                                                ToonifyError::Error(_) => print!("Error: {}", err),
                                                                ToonifyError::Null(_) => print!("Error: Something went wrong")
                                                            }
                                                        }
                                                    },
                                                    None => print!("Error: Image name is missing"),
                                                }
                                            },
                                            None => print!("Error: Path is missing")
                                        }
                                    } else {
                                        print!("Error: {} is a wrong option", arg_four)
                                    },
                                    None => {
                                        let toon = ToonifyFile::new(arg_three,arg_two);
                                        match toon.image() {
                                            Ok(image) => print!("{}",image),
                                            Err(err) => match err {
                                                ToonifyError::Error(_) => print!("Error: {}", err),
                                                ToonifyError::Null(_) => print!("Error: Something went wrong")
                                            }
                                        }
                                    }
                                }
                            },
                            None => print!("Error: Image file is missing")
                        }
                    },
                    None => print!("Error: API Key is missing")
                }
            } else if arg_one.eq("-URI") {
                match args.get(2) {
                    Some(arg_two) => {
                        match args.get(3) {
                            Some(arg_three) => {
                                match args.get(4) {
                                    Some(arg_four) => if arg_four.eq("--save") {
                                        match args.get(5) {
                                            Some(arg_five) => {
                                                match args.get(6) {
                                                    Some(arg_six) => {
                                                        let toon = Toonify::new(arg_three,arg_two);
                                                        match toon.image() {
                                                            Ok(image) => {
                                                                match save_image(image.as_str(),arg_five,
                                                                                 arg_six) {
                                                                    Ok(_) => print!("Image saved successfully."),
                                                                    Err(err) => if err.eq("null") {
                                                                        print!("Error: Something went wrong")
                                                                    } else {
                                                                        print!("Error: {}", err)
                                                                    }
                                                                }
                                                            },
                                                            Err(err) => match err {
                                                                ToonifyError::Error(_) => print!("Error: {}", err),
                                                                ToonifyError::Null(_) => print!("Error: Something went wrong")
                                                            }
                                                        }
                                                    },
                                                    None => print!("Error: Image name is missing"),
                                                }
                                            },
                                            None => print!("Error: Path is missing")
                                        }
                                    } else {
                                        print!("Error: {} is a wrong option", arg_four)
                                    },
                                    None => {
                                        let toon = Toonify::new(arg_three,arg_two);
                                        match toon.image() {
                                            Ok(image) => print!("{}",image),
                                            Err(err) => match err {
                                                ToonifyError::Error(_) => print!("Error: {}", err),
                                                ToonifyError::Null(_) => print!("Error: Something went wrong")
                                            }
                                        }
                                    }
                                }
                            },
                            None => print!("Error: Image URI is missing")
                        }
                    },
                    None => print!("Error: API Key is missing")
                }
            } else {
                print!("Usage: toonify -FILE \"API-KEY\" \"path-to-image\" [-options]\n");
                print!("           (Photo of any face into a cartoon from given image)\n");
                print!("   or  toonify -URI \"API-KEY\" \"image-URI\" [-options]\n");
                print!("           (Photo of any face into a cartoon from given image URI)\n");
                print!("where options include:\n");
                print!("    --save \"path\" \"image-name\"          save image")
            },
            None => {
                print!("Usage: toonify -FILE \"API-KEY\" \"path-to-image\" [-options]\n");
                print!("           (Photo of any face into a cartoon from given image)\n");
                print!("   or  toonify -URI \"API-KEY\" \"image-URI\" [-options]\n");
                print!("           (Photo of any face into a cartoon from given image URI)\n");
                print!("where options include:\n");
                print!("    --save \"path\" \"image-name\"          save image")
            }
        }
    }
}

fn http(image_uri: &str) -> Option<Response> {
    match reqwest::blocking::Client::new().get(image_uri)
        .send() {
        Ok(data) => Some(data),
        Err(_) => None
    }
}

fn save_image(image_uri: &str, path: &str, image_name: &str) -> Result<(), String> {
    match http(image_uri) {
        Some(response) => match response.bytes() {
            Ok(bytes) => match std::fs::File::create(format!("{}/{}.png", path, image_name)) {
                Ok(mut file) => {
                    let mut content = Cursor::new(bytes);
                    match std::io::copy(&mut content, &mut file) {
                        Ok(_) => Ok(()),
                        Err(_) => Err("null".to_string())
                    }
                },
                Err(err) => Err(err.to_string()),
            },
            Err(_) => Err("null".to_string())
        },
        None => Err("null".to_string())
    }
}