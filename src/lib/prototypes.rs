use crate::{HashInfo, Mode, Prototype};
use lazy_static::lazy_static;
use regex::Regex;
lazy_static! {
        pub static ref PROTOTYPES: Vec<Prototype> = vec![
                Prototype {
                    regex: Regex::new(r"(?i)^[a-f0-9]{4}$").unwrap(),
                    modes: HashInfo {
                        modes: vec![
                            Mode {
                                john: None,
                                hashcat: None,
                                extended: false,
                                name: "CRC-16".to_string()
                            },
                            Mode {
                                john: None,
                                hashcat: None,
                                extended: false,
                                name: "CRC-16-CCITT".to_string()
                            },
                            Mode {
                                john: None,
                                hashcat: None,
                                extended: false,
                                name: "FCS-16".to_string()
                            }
                        ]
           }},
            Prototype {
                regex: Regex::new(r"(?i)^[a-f0-9]{8}$").unwrap(),
                modes: HashInfo {
 modes: vec![
                    Mode {
                        john: None,
                        hashcat: None,
                        extended: false,
                        name: "Adler-32".to_string()
                    },
                    Mode {
                        john: None,
                        hashcat: None,
                        extended: false,
                        name: "CRC-32B".to_string()
                    },
                    Mode {
                        john: None,
                        hashcat: None,
                        extended: false,
                        name: "FCS-32".to_string()
                    },
                    Mode {
                        john: None,
                        hashcat: None,
                        extended: false,
                        name: "GHash-32-3".to_string()
                    },
                    Mode {
                        john: None,
                        hashcat: None,
                        extended: false,
                        name: "GHash-32-5".to_string()
                    },
                    Mode {
                        john: None,
                        hashcat: None,
                        extended: false,
                        name: "FNV-132".to_string()
                    },
                    Mode {
                        john: None,
                        hashcat: None,
                        extended: false,
                        name: "Fletcher-32".to_string()
                    },
                    Mode {
                        john: None,
                        hashcat: None,
                        extended: false,
                        name: "Joaat".to_string()
                    },
                    Mode {
                        john: None,
                        hashcat: None,
                        extended: false,
                        name: "ELF-32".to_string()
                    },
                    Mode {
        john: None,
                        hashcat: None,
                        extended: false,
                        name: "XOR-32".to_string()
                    }
                ]
           }},
            Prototype {
        regex: Regex::new(r"(?i)^[a-f0-9]{6}$").unwrap(),
                modes: HashInfo {
 modes: vec![
Mode {
        john: None,
                        hashcat: None,
                        extended: false,
                        name: "CRC-24".to_string()
                    }
                ]
           }},
            Prototype {
        regex: Regex::new(r"(?i)^(\$crc32\$[a-f0-9]{8}.)?[a-f0-9]{8}$").unwrap(),
                modes: HashInfo {
 modes: vec![
Mode {
        john: Some("crc32".to_string()),
                        hashcat: None,
                        extended: false,
                        name: "CRC-32".to_string()
                    }
                ]
           }},
            Prototype {
        regex: Regex::new(r"(?i)^\+[a-z0-9/.]{12}$").unwrap(),
                modes: HashInfo {
 modes: vec![
Mode {
        john: Some("bfegg".to_string()),
                        hashcat: None,
                        extended: false,
                        name: "Eggdrop IRC Bot".to_string()
                    }
                ]
           }},
            Prototype {
        regex: Regex::new(r"(?i)^[a-z0-9/.]{13}$").unwrap(),
                modes: HashInfo {
 modes: vec![
Mode {
        john: Some("descrypt".to_string()),
                        hashcat: Some(1500),
                        extended: false,
                        name: "DES(Unix)".to_string()
                    },
                    Mode {
        john: Some("descrypt".to_string()),
                        hashcat: Some(1500),
                        extended: false,
                        name: "Traditional DES".to_string()
                    },
                    Mode {
        john: Some("descrypt".to_string()),
                        hashcat: Some(1500),
                        extended: false,
                        name: "DEScrypt".to_string()
                    }
                ]
            }},
            Prototype {
        regex: Regex::new(r"(?i)^[a-f0-9]{16}$").unwrap(),
                modes: HashInfo {
 modes: vec![
Mode {
        john: Some("mysql".to_string()),
                        hashcat: Some(200),
                        extended: false,
                        name: "MySQL323".to_string()
                    },
                    Mode {
        john: None,
                        hashcat: Some(3100),
                        extended: false,
                        name: "DES(Oracle)".to_string()
                    },
                    Mode {
        john: None,
                        hashcat: Some(5100),
                        extended: false,
                        name: "Half MD5".to_string()
                    },
                    Mode {
        john: None,
                        hashcat: Some(3100),
                        extended: false,
                        name: "Oracle 7-10g".to_string()
                    },
                    Mode {
        john: None,
                        hashcat: None,
                        extended: false,
                        name: "FNV-164".to_string()
                    },
                    Mode {
        john: None,
                        hashcat: None,
                        extended: false,
                        name: "CRC-64".to_string()
                    }
                ]
           }},
            Prototype {
        regex: Regex::new(r"(?i)^[a-z0-9/.]{16}$").unwrap(),
                modes: HashInfo {
 modes: vec![
Mode {
        john: Some("pix-md5".to_string()),
                        hashcat: Some(2400),
                        extended: false,
                        name: "Cisco-PIX(MD5)".to_string()
                    }
                ]
           }},
            Prototype {
        regex: Regex::new(r"(?i)^\([a-z0-9/+]{20}\)$").unwrap(),
                modes: HashInfo {
 modes: vec![
Mode {
        john: Some("dominosec".to_string()),
                        hashcat: Some(8700),
                        extended: false,
                        name: "Lotus Notes/Domino 6".to_string()
                    }
                ]
           }},
            Prototype {
        regex: Regex::new(r"(?i)^_[a-z0-9/.]{19}$").unwrap(),
                modes: HashInfo {
 modes: vec![
Mode {
        john: Some("bsdicrypt".to_string()),
                        hashcat: None,
                        extended: false,
                        name: "BSDi Crypt".to_string()
                    }
                ]
           }},
            Prototype {
        regex: Regex::new(r"(?i)^[a-f0-9]{24}$").unwrap(),
                modes: HashInfo {
 modes: vec![
Mode {
        john: None,
                        hashcat: None,
                        extended: false,
                        name: "CRC-96(ZIP)".to_string()
                    }
                ]
           }},
            Prototype {
        regex: Regex::new(r"(?i)^[a-z0-9/.]{24}$").unwrap(),
                modes: HashInfo {
 modes: vec![
Mode {
        john: None,
                        hashcat: None,
                        extended: false,
                        name: "Crypt16".to_string()
                    }
                ]
           }},
            Prototype {
        regex: Regex::new(r"(?i)^(\$md2\$)?[a-f0-9]{32}$").unwrap(),
                modes: HashInfo {
 modes: vec![
Mode {
        john: Some("md2".to_string()),
                        hashcat: None,
                        extended: false,
                        name: "MD2".to_string()
                    }
                ]
           }},
            Prototype {
        regex: Regex::new(r"(?i)^[a-f0-9]{32}(:.+)?$").unwrap(),
                modes: HashInfo {
 modes: vec![
Mode {
        john: Some("raw-md5".to_string()),
                        hashcat: Some(0),
                        extended: false,
                        name: "MD5".to_string()
                   },
                    Mode {
        john: Some("raw-md4".to_string()),
                        hashcat: Some(900),
                        extended: false,
                        name: "MD4".to_string()
                    },
                    Mode {
        john: None,
                        hashcat: Some(2600),
                        extended: false,
                        name: "Double MD5".to_string()
                    },
                    Mode {
        john: Some("lm".to_string()),
                        hashcat: Some(3000),
                        extended: false,
                        name: "LM".to_string()
                    },
                    Mode {
        john: Some("ripemd-128".to_string()),
                        hashcat: None,
                        extended: false,
                        name: "RIPEMD-128".to_string()
                    },
                    Mode {
        john: Some("haval-128-4".to_string()),
                        hashcat: None,
                        extended: false,
                        name: "Haval-128".to_string()
                    },
                    Mode {
        john: None,
                        hashcat: None,
                        extended: false,
                        name: "Tiger-128".to_string()
                    },
                    Mode {
        john: None,
                        hashcat: None,
                        extended: false,
                        name: "Skein-256(128)".to_string()
                    },
                    Mode {
        john: None,
                        hashcat: None,
                        extended: false,
                        name: "Skein-512(128)".to_string()
                    },
                    Mode {
        john: Some("lotus5".to_string()),
                        hashcat: Some(8600),
                        extended: false,
                        name: "Lotus Notes/Domino 5".to_string()
                    },
                    Mode {
        john: None,
                        hashcat: Some(23),
                        extended: false,
                        name: "Skype".to_string()
                    },
                    Mode {
        john: None,
                        hashcat: None,
                        extended: true,
                        name: "ZipMonster".to_string()
                    },
                    Mode {
        john: None,
                        hashcat: Some(11000),
                        extended: true,
                        name: "PrestaShop".to_string()
                    },
                    Mode {
        john: None,
                        hashcat: Some(3500),
                        extended: true,
                        name: "md5(md5(md5($pass)))".to_string()
                    },
                    Mode {
        john: None,
                        hashcat: Some(4300),
                        extended: true,
                        name: "md5(strtoupper(md5($pass)))".to_string()
                    },
                    Mode {
        john: None,
                        hashcat: Some(4400),
                        extended: true,
                        name: "md5(sha1($pass))".to_string()
                    },
                    Mode {
        john: None,
                        hashcat: Some(10),
                        extended: true,
                        name: "md5($pass.$salt)".to_string()
                    },
                    Mode {
        john: None,
                        hashcat: Some(20),
                        extended: true,
                        name: "md5($salt.$pass)".to_string()
                    },
                    Mode {
        john: None,
                        hashcat: Some(30),
                        extended: true,
                        name: "md5(unicode($pass).$salt)".to_string()
                    },
                    Mode {
        john: None,
                        hashcat: Some(40),
                        extended: true,
                        name: "md5($salt.unicode($pass))".to_string()
                    },
                    Mode {
        john: Some("hmac-md5".to_string()),
                        hashcat: Some(50),
                        extended: true,
                        name: "HMAC-MD5 (key = $pass)".to_string()
                    },
                    Mode {
        john: Some("hmac-md5".to_string()),
                        hashcat: Some(60),
                        extended: true,
                        name: "HMAC-MD5 (key = $salt)".to_string()
                    },
                    Mode {
        john: None,
                        hashcat: Some(3610),
                        extended: true,
                        name: "md5(md5($salt).$pass)".to_string()
                    },
                    Mode {
        john: None,
                        hashcat: Some(3710),
                        extended: true,
                        name: "md5($salt.md5($pass))".to_string()
                    },
                    Mode {
        john: None,
                        hashcat: Some(3720),
                        extended: true,
                        name: "md5($pass.md5($salt))".to_string()
                    },
                    Mode {
        john: None,
                        hashcat: Some(3810),
                        extended: true,
                        name: "md5($salt.$pass.$salt)".to_string()
                    },
                    Mode {
        john: None,
                        hashcat: Some(3910),
                        extended: true,
                        name: "md5(md5($pass).md5($salt))".to_string()
                    },
                    Mode {
        john: None,
                        hashcat: Some(4010),
                        extended: true,
                        name: "md5($salt.md5($salt.$pass))".to_string()
                    },
                    Mode {
        john: None,
                        hashcat: Some(4110),
                        extended: true,
                        name: "md5($salt.md5($pass.$salt))".to_string()
                    },
                    Mode {
        john: None,
                        hashcat: Some(4210),
                        extended: true,
                        name: "md5($username.0.$pass)".to_string()
                    }
                ]
           }},
            Prototype {
        regex: Regex::new(r"(?i)^(\$snefru\$)?[a-f0-9]{32}$").unwrap(),
                modes: HashInfo {
 modes: vec![
Mode {
        john: Some("snefru-128".to_string()),
                        hashcat: None,
                        extended: false,
                        name: "Snefru-128".to_string()
                    }
                ]
           }},
            Prototype {
        regex: Regex::new(r"(?i)^(\$NT\$)?[a-f0-9]{32}$").unwrap(),
                modes: HashInfo {
 modes: vec![
Mode {
        john: Some("nt".to_string()),
                        hashcat: Some(1000),
                        extended: false,
                        name: "NTLM".to_string()
                    }
                ]
           }},
            Prototype {
        regex: Regex::new(r"(?i)^([^\\/:*?<>|]{1,20}:)?[a-f0-9]{32}(:[^\\/:*?<>|]{1,20})?$").unwrap(),
                modes: HashInfo {
 modes: vec![
Mode {
        john: Some("mscach".to_string()),
                        hashcat: Some(1100),
                        extended: false,
                        name: "Domain Cached Credentials".to_string()
                    }
                ]
           }},
            Prototype {
        regex: Regex::new(r"(?i)^([^\\/:*?<>|]{1,20}:)?(\$DCC2\$10240#[^\\/:*?<>|]{1,20}#)?[a-f0-9]{32}$").unwrap(),
                modes: HashInfo {
 modes: vec![
Mode {
        john: Some("mscach2".to_string()),
                        hashcat: Some(2100),
                        extended: false,
                        name: "Domain Cached Credentials 2".to_string()
                    }
                ]
           }},
            Prototype {
        regex: Regex::new(r"^SHA[a-z0-9A-Z/+]{27}=$").unwrap(),
                modes: HashInfo {
 modes: vec![
Mode {
        john: Some("nsldap".to_string()),
                        hashcat: Some(101),
                        extended: false,
                        name: "SHA-1(Base64)".to_string()
                    },
                    Mode {
        john: Some("nsldap".to_string()),
                        hashcat: Some(101),
                        extended: false,
                        name: "Netscape LDAP SHA".to_string()
                    }
                ]
           }},
            Prototype {
        regex: Regex::new(r"(?i)^\$1\$[a-z0-9/.]{0,8}\$[a-z0-9/.]{22}(:.*)?$").unwrap(),
                modes: HashInfo {
 modes: vec![
Mode {
        john: Some("md5crypt".to_string()),
                        hashcat: Some(500),
                        extended: false,
                        name: "MD5 Crypt".to_string()
                    },
                    Mode {
        john: Some("md5crypt".to_string()),
                        hashcat: Some(500),
                        extended: false,
                        name: "Cisco-IOS(MD5)".to_string()
                    },
                    Mode {
        john: Some("md5crypt".to_string()),
                        hashcat: Some(500),
                        extended: false,
                        name: "FreeBSD MD5".to_string()
                    }
                ]
           }},
            Prototype {
        regex: Regex::new(r"(?i)^0x[a-f0-9]{32}$").unwrap(),
                modes: HashInfo {
 modes: vec![
Mode {
        john: None,
                        hashcat: None,
                        extended: false,
                        name: "Lineage II C4".to_string()
                    }
                ]
           }},
            Prototype {
        regex: Regex::new(r"(?i)^\$H\$[a-z0-9/.]{31}$").unwrap(),
                modes: HashInfo {
 modes: vec![
Mode {
        john: Some("phpass".to_string()),
                        hashcat: Some(400),
                        extended: false,
                        name: "phpBB v3.x".to_string()
                    },
                    Mode {
        john: Some("phpass".to_string()),
                        hashcat: Some(400),
                        extended: false,
                        name: "Wordpress v2.6.0/2.6.1".to_string()
                    },
                    Mode {
        john: Some("phpass".to_string()),
                        hashcat: Some(400),
                        extended: false,
                        name: "PHPass' Portable Hash".to_string()
                    }
                ]
            }},
            Prototype {
        regex: Regex::new(r"(?i)^\$P\$[a-z0-9/.]{31}$").unwrap(),
                modes: HashInfo {
 modes: vec![
Mode {
        john: Some("phpass".to_string()),
                        hashcat: Some(400),
                        extended: false,
                        name: "Wordpress \u{2265} v2.6.2".to_string()
                    },
                    Mode {
        john: Some("phpass".to_string()),
                        hashcat: Some(400),
                        extended: false,
                        name: "Joomla \u{2265} v2.5.18".to_string()
                    },
                    Mode {
        john: Some("phpass".to_string()),
                        hashcat: Some(400),
                        extended: false,
                        name: "PHPass' Portable Hash".to_string()
                    }
                ]
           }},
            Prototype {
        regex: Regex::new(r"(?i)^[a-f0-9]{32}:[a-z0-9]{2}$").unwrap(),
                modes: HashInfo {
 modes: vec![
Mode {
        john: None,
                        hashcat: Some(21),
                        extended: false,
                        name: "osCommerce".to_string()
                    },
                    Mode {
        john: None,
                        hashcat: Some(21),
                        extended: false,
                        name: "xt:Commerce".to_string()
                    }
                ]
           }},
            Prototype {
        regex: Regex::new(r"(?i)^\$apr1\$[a-z0-9/.]{0,8}\$[a-z0-9/.]{22}$").unwrap(),
                modes: HashInfo {
 modes: vec![
Mode {
        john: None,
                        hashcat: Some(1600),
                        extended: false,
                        name: "MD5(APR)".to_string()
                    },
                    Mode {
        john: None,
                        hashcat: Some(1600),
                        extended: false,
                        name: "Apache MD5".to_string()
                    },
                    Mode {
        john: None,
                        hashcat: Some(1600),
                        extended: true,
                        name: "md5apr1".to_string()
                    }
                ]
           }},
            Prototype {
        regex: Regex::new(r"smd5[a-z0-9A-Z$/.]{31}$").unwrap(),
                modes: HashInfo {
 modes: vec![
Mode {
        john: Some("aix-smd5".to_string()),
                        hashcat: Some(6300),
                        extended: false,
                        name: "AIX(smd5)".to_string()
                    }
                ]
           }},
            Prototype {
        regex: Regex::new(r"(?i)^[a-f0-9]{32}:[a-f0-9]{32}$").unwrap(),
                modes: HashInfo {
 modes: vec![
Mode {
        john: None,
                        hashcat: Some(3721),
                        extended: false,
                        name: "WebEdition CMS".to_string()
                    }
                ]
           }},
            Prototype {
        regex: Regex::new(r"(?i)^[a-f0-9]{32}:.{5}$").unwrap(),
                modes: HashInfo {
 modes: vec![
Mode {
        john: None,
                        hashcat: Some(2811),
                        extended: false,
                        name: "IP.Board \u{2265} v2+".to_string()
                    }
                ]
           }},
            Prototype {
        regex: Regex::new(r"(?i)^[a-f0-9]{32}:.{8}$").unwrap(),
                modes: HashInfo {
 modes: vec![
Mode {
        john: None,
                        hashcat: Some(2811),
                        extended: false,
                        name: "MyBB \u{2265} v1.2+".to_string()
                    }
                ]
           }},
            Prototype {
        regex: Regex::new(r"(?i)^[a-z0-9]{34}$").unwrap(),
                modes: HashInfo {
 modes: vec![
Mode {
        john: None,
                        hashcat: None,
                        extended: false,
                        name: "CryptoCurrency(Adress)".to_string()
                    }
                ]
           }},
            Prototype {
        regex: Regex::new(r"(?i)^[a-f0-9]{40}(:.+)?$").unwrap(),
                modes: HashInfo {
 modes: vec![
Mode {
        john: Some("raw-sha1".to_string()),
                        hashcat: Some(100),
                        extended: false,
                        name: "SHA-1".to_string()
                    },
                    Mode {
        john: None,
                        hashcat: Some(4500),
                        extended: false,
                        name: "Double SHA-1".to_string()
                    },
                    Mode {
        john: Some("ripemd-160".to_string()),
                        hashcat: Some(6000),
                        extended: false,
                        name: "RIPEMD-160".to_string()
                    },
                    Mode {
        john: None,
                        hashcat: None,
                        extended: false,
                        name: "Haval-160".to_string()
                    },
                    Mode {
        john: None,
                        hashcat: None,
                        extended: false,
                        name: "Tiger-160".to_string()
                    },
                    Mode {
        john: None,
                        hashcat: None,
                        extended: false,
                        name: "HAS-160".to_string()
                    },
                    Mode {
        john: Some("raw-sha1-linkedin".to_string()),
                        hashcat: Some(190),
                        extended: false,
                        name: "LinkedIn".to_string()
                    },
                    Mode {
        john: None,
                        hashcat: None,
                        extended: false,
                        name: "Skein-256(160)".to_string()
                    },
                    Mode {
        john: None,
                        hashcat: None,
                        extended: false,
                        name: "Skein-512(160)".to_string()
                    },
                    Mode {
        john: None,
                        hashcat: None,
                        extended: true,
                        name: "MangosWeb Enhanced CMS".to_string()
                    },
                    Mode {
        john: None,
                        hashcat: Some(4600),
                        extended: true,
                        name: "sha1(sha1(sha1($pass)))".to_string()
                    },
                    Mode {
        john: None,
                        hashcat: Some(4700),
                        extended: true,
                        name: "sha1(md5($pass))".to_string()
                    },
                    Mode {
        john: None,
                        hashcat: Some(110),
                        extended: true,
                        name: "sha1($pass.$salt)".to_string()
                    },
                    Mode {
        john: None,
                        hashcat: Some(120),
                        extended: true,
                        name: "sha1($salt.$pass)".to_string()
                    },
                    Mode {
        john: None,
                        hashcat: Some(130),
                        extended: true,
                        name: "sha1(unicode($pass).$salt)".to_string()
                    },
                    Mode {
        john: None,
                        hashcat: Some(140),
                        extended: true,
                        name: "sha1($salt.unicode($pass))".to_string()
                    },
                    Mode {
        john: Some("hmac-sha1".to_string()),
                        hashcat: Some(150),
                        extended: true,
                        name: "HMAC-SHA1 (key = $pass)".to_string()
                    },
                    Mode {
        john: Some("hmac-sha1".to_string()),
                        hashcat: Some(160),
                        extended: true,
                        name: "HMAC-SHA1 (key = $salt)".to_string()
                    },
                    Mode {
        john: None,
                        hashcat: Some(4710),
                        extended: true,
                        name: "sha1($salt.$pass.$salt)".to_string()
                    }
                ]
           }},
            Prototype {
        regex: Regex::new(r"(?i)^\*[a-f0-9]{40}$").unwrap(),
                modes: HashInfo {
 modes: vec![
Mode {
        john: Some("mysql-sha1".to_string()),
                        hashcat: Some(300),
                        extended: false,
                        name: "MySQL5.x".to_string()
                    },
                    Mode {
        john: Some("mysql-sha1".to_string()),
                        hashcat: Some(300),
                        extended: false,
                        name: "MySQL4.1".to_string()
                    }
                ]
           }},
            Prototype {
        regex: Regex::new(r"(?i)^[a-z0-9]{43}$").unwrap(),
                modes: HashInfo {
 modes: vec![
Mode {
        john: None,
                        hashcat: Some(5700),
                        extended: false,
                        name: "Cisco-IOS(SHA-256)".to_string()
                    }
                ]
           }},
            Prototype {
        regex: Regex::new(r"^SSHA[a-z0-9A-Z/+]{38}==$").unwrap(),
                modes: HashInfo {
 modes: vec![
Mode {
        john: Some("nsldaps".to_string()),
                        hashcat: Some(111),
                        extended: false,
                        name: "SSHA-1(Base64)".to_string()
                    },
                    Mode {
        john: Some("nsldaps".to_string()),
                        hashcat: Some(111),
                        extended: false,
                        name: "Netscape LDAP SSHA".to_string()
                    },
                    Mode {
        john: Some("nsldaps".to_string()),
                        hashcat: Some(111),
                        extended: true,
                        name: "nsldaps".to_string()
                    }
                ]
           }},
            Prototype {
        regex: Regex::new(r"(?i)^[a-z0-9=]{47}$").unwrap(),
                modes: HashInfo {
 modes: vec![
Mode {
        john: Some("fortigate".to_string()),
                        hashcat: Some(7000),
                        extended: false,
                        name: "Fortigate(FortiOS)".to_string()
                    }
                ]
           }},
            Prototype {
        regex: Regex::new(r"(?i)^[a-f0-9]{48}$").unwrap(),
                modes: HashInfo {
 modes: vec![
Mode {
        john: None,
                        hashcat: None,
                        extended: false,
                        name: "Haval-192".to_string()
                    },
                    Mode {
        john: Some("tiger".to_string()),
                        hashcat: None,
                        extended: false,
                        name: "Tiger-192".to_string()
                    },
                    Mode {
        john: None,
                        hashcat: None,
                        extended: false,
                        name: "SHA-1(Oracle)".to_string()
                    },
                    Mode {
        john: Some("xsha".to_string()),
                        hashcat: Some(122),
                        extended: false,
                        name: "OSX v10.4".to_string()
                    },
                    Mode {
        john: Some("xsha".to_string()),
                        hashcat: Some(122),
                        extended: false,
                        name: "OSX v10.5".to_string()
                    },
                    Mode {
        john: Some("xsha".to_string()),
                        hashcat: Some(122),
                        extended: false,
                        name: "OSX v10.6".to_string()
                    }
                ]
           }},
            Prototype {
        regex: Regex::new(r"(?i)^[a-f0-9]{51}$").unwrap(),
                modes: HashInfo {
 modes: vec![
Mode {
        john: None,
                        hashcat: None,
                        extended: false,
                        name: "Palshop CMS".to_string()
                    }
                ]
           }},
            Prototype {
        regex: Regex::new(r"(?i)^[a-z0-9]{51}$").unwrap(),
                modes: HashInfo {
 modes: vec![
Mode {
        john: None,
                        hashcat: None,
                        extended: false,
                        name: "CryptoCurrency(PrivateKey)".to_string()
                    }
                ]
           }},
            Prototype {
        regex: Regex::new(r"^ssha1[0-9]{2}\$[a-z0-9$/.]{44}$").unwrap(),
                modes: HashInfo {
 modes: vec![
Mode {
        john: Some("aix-ssha1".to_string()),
                        hashcat: Some(6700),
                        extended: false,
                        name: "AIX(ssha1)".to_string()
                    }
                ]
           }},
            Prototype {
        regex: Regex::new(r"(?i)^0x0100[a-f0-9]{48}$").unwrap(),
                modes: HashInfo {
 modes: vec![
Mode {
        john: Some("mssql05".to_string()),
                        hashcat: Some(132),
                        extended: false,
                        name: "MSSQL(2005)".to_string()
                    },
                    Mode {
        john: Some("mssql05".to_string()),
                        hashcat: Some(132),
                        extended: false,
                        name: "MSSQL(2008)".to_string()
                    }
                ]
            }},
            Prototype {
        regex: Regex::new(r"(?i)^(\$md5,rounds=[0-9]+\$|\$md5\$rounds=[0-9]+\$|\$md5\$)[a-z0-9/.]{0,16}(\$|\$\$)[a-z0-9/.]{22}$").unwrap(),
                modes: HashInfo {
 modes: vec![
Mode {
        john: Some("sunmd5".to_string()),
                        hashcat: Some(3300),
                        extended: false,
                        name: "Sun MD5 Crypt".to_string()
                    }
                ]
           }},
            Prototype {
        regex: Regex::new(r"(?i)^[a-f0-9]{56}$").unwrap(),
                modes: HashInfo {
 modes: vec![
Mode {
        john: Some("raw-sha224".to_string()),
                        hashcat: None,
                        extended: false,
                        name: "SHA-224".to_string()
                    },
                    Mode {
        john: None,
                        hashcat: None,
                        extended: false,
                        name: "Haval-224".to_string()
                    },
                    Mode {
        john: None,
                        hashcat: None,
                        extended: false,
                        name: "SHA3-224".to_string()
                    },
                    Mode {
        john: None,
                        hashcat: None,
                        extended: false,
                        name: "Skein-256(224)".to_string()
                    },
                    Mode {
        john: None,
                        hashcat: None,
                        extended: false,
                        name: "Skein-512(224)".to_string()
                    }
                ]
           }},
            Prototype {
        regex: Regex::new(r"(?i)^(\$2[axy]|\$2)\$[0-9]{2}\$[a-z0-9/.]{53}$").unwrap(),
                modes: HashInfo {
 modes: vec![
Mode {
        john: Some("bcrypt".to_string()),
                        hashcat: Some(3200),
                        extended: false,
                        name: "Blowfish(OpenBSD)".to_string()
                    },
                    Mode {
        john: None,
                        hashcat: None,
                        extended: false,
                        name: "Woltlab Burning Board 4.x".to_string()
                    },
                    Mode {
        john: Some("bcrypt".to_string()),
                        hashcat: Some(3200),
                        extended: false,
                        name: "bcrypt".to_string()
                    }
                ]
           }},
            Prototype {
        regex: Regex::new(r"(?i)^[a-f0-9]{40}:[a-f0-9]{16}$").unwrap(),
                modes: HashInfo {
 modes: vec![
Mode {
        john: None,
                        hashcat: Some(5800),
                        extended: false,
                        name: "Android PIN".to_string()
                    }
                ]
           }},
            Prototype {
        regex: Regex::new(r"(?i)^(S:)?[a-f0-9]{40}(:)?[a-f0-9]{20}$").unwrap(),
                modes: HashInfo {
 modes: vec![
Mode {
        john: Some("oracle11".to_string()),
                        hashcat: Some(112),
                        extended: false,
                        name: "Oracle 11g/12c".to_string()
                    }
                ]
           }},
            Prototype {
        regex: Regex::new(r"(?i)^\$bcrypt-sha256\$(2[axy]|2),[0-9]+\$[a-z0-9.]{22}\$[a-z0-9/.]{31}$").unwrap(),
                modes: HashInfo {
 modes: vec![
Mode {
        john: None,
                        hashcat: None,
                        extended: false,
                        name: "bcrypt(SHA-256)".to_string()
                    }
                ]
           }},
            Prototype {
        regex: Regex::new(r"(?i)^[a-f0-9]{32}:.{3}$").unwrap(),
                modes: HashInfo {
 modes: vec![
Mode {
        john: None,
                        hashcat: Some(2611),
                        extended: false,
                        name: "vBulletin < v3.8.5".to_string()
                    }
                ]
           }},
            Prototype {
        regex: Regex::new(r"(?i)^[a-f0-9]{32}:.{30}$").unwrap(),
                modes: HashInfo {
 modes: vec![
Mode {
        john: None,
                        hashcat: Some(2711),
                        extended: false,
                        name: "vBulletin \u{2265} v3.8.5".to_string()
                    }
                ]
           }},
            Prototype {
        regex: Regex::new(r"(?i)^(\$snefru\$)?[a-f0-9]{64}$").unwrap(),
                modes: HashInfo {
 modes: vec![
Mode {
        john: Some("snefru-256".to_string()),
                        hashcat: None,
                        extended: false,
                        name: "Snefru-256".to_string()
                    }
                ]
           }},
            Prototype {
        regex: Regex::new(r"(?i)^[a-f0-9]{64}(:.+)?$").unwrap(),
                modes: HashInfo {
 modes: vec![
Mode {
        john: Some("raw-sha256".to_string()),
                        hashcat: Some(1400),
                        extended: false,
                        name: "SHA-256".to_string()
                    },
                    Mode {
        john: None,
                        hashcat: None,
                        extended: false,
                        name: "RIPEMD-256".to_string()
                    },
                    Mode {
        john: Some("haval-256-3".to_string()),
                        hashcat: None,
                        extended: false,
                        name: "Haval-256".to_string()
                    },
                    Mode {
        john: Some("gost".to_string()),
                        hashcat: Some(6900),
                        extended: false,
                        name: "GOST R 34.11-94".to_string()
                    },
                    Mode {
        john: None,
                        hashcat: None,
                        extended: false,
                        name: "GOST CryptoPro S-Box".to_string()
                    },
                    Mode {
        john: Some("raw-keccak-256".to_string()),
                        hashcat: Some(5000),
                        extended: false,
                        name: "SHA3-256".to_string()
                    },
                    Mode {
        john: Some("skein-256".to_string()),
                        hashcat: None,
                        extended: false,
                        name: "Skein-256".to_string()
                    },
                    Mode {
        john: None,
                        hashcat: None,
                        extended: false,
                        name: "Skein-512(256)".to_string()
                    },
                    Mode {
        john: None,
                        hashcat: None,
                        extended: true,
                        name: "Ventrilo".to_string()
                    },
                    Mode {
        john: None,
                        hashcat: Some(1410),
                        extended: true,
                        name: "sha256($pass.$salt)".to_string()
                    },
                    Mode {
        john: None,
                        hashcat: Some(1420),
                        extended: true,
                        name: "sha256($salt.$pass)".to_string()
                    },
                    Mode {
        john: None,
                        hashcat: Some(1430),
                        extended: true,
                        name: "sha256(unicode($pass).$salt)".to_string()
                    },
                    Mode {
        john: None,
                        hashcat: Some(1440),
                        extended: true,
                        name: "sha256($salt.unicode($pass))".to_string()
                    },
                    Mode {
        john: Some("hmac-sha256".to_string()),
                        hashcat: Some(1450),
                        extended: true,
                        name: "HMAC-SHA256 (key = $pass)".to_string()
                    },
                    Mode {
        john: Some("hmac-sha256".to_string()),
                        hashcat: Some(1460),
                        extended: true,
                        name: "HMAC-SHA256 (key = $salt)".to_string()
                    }
                ]
           }},
            Prototype {
        regex: Regex::new(r"(?i)^[a-f0-9]{32}:[a-z0-9]{32}$").unwrap(),
                modes: HashInfo {
 modes: vec![
Mode {
        john: None,
                        hashcat: Some(11),
                        extended: false,
                        name: "Joomla < v2.5.18".to_string()
                    }
                ]
           }},
            Prototype {
        regex: Regex::new(r"(?i)^[a-f-0-9]{32}:[a-f-0-9]{32}$").unwrap(),
                modes: HashInfo {
 modes: vec![
Mode {
        john: None,
                        hashcat: None,
                        extended: false,
                        name: "SAM(LM_Hash:NT_Hash)".to_string()
                    }
                ]
           }},
            Prototype {
        regex: Regex::new(r"(?i)^(\$chap\$0\*)?[a-f0-9]{32}[*:][a-f0-9]{32}(:[0-9]{2})?$").unwrap(),
                modes: HashInfo {
 modes: vec![
Mode {
        john: Some("chap".to_string()),
                        hashcat: Some(4800),
                        extended: false,
                        name: "MD5(Chap)".to_string()
                   },
                    Mode {
        john: Some("chap".to_string()),
                        hashcat: Some(4800),
                        extended: false,
                        name: "iSCSI CHAP Authentication".to_string()
                    }
                ]
           }},
            Prototype {
        regex: Regex::new(r"(?i)^\$episerver\$\*0\*[a-z0-9/=+]+\*[a-z0-9/=+]{27,28}$").unwrap(),
                modes: HashInfo {
 modes: vec![
Mode {
        john: Some("episerver".to_string()),
                        hashcat: Some(141),
                        extended: false,
                        name: "EPiServer 6.x < v4".to_string()
                    }
                ]
           }},
            Prototype {
        regex: Regex::new(r"^ssha256[0-9]{2}\$[a-z0-9$/.]{60}$").unwrap(),
                modes: HashInfo {
 modes: vec![
Mode {
        john: Some("aix-ssha256".to_string()),
                        hashcat: Some(6400),
                        extended: false,
                        name: "AIX(ssha256)".to_string()
                    }
                ]
           }},
            Prototype {
        regex: Regex::new(r"(?i)^[a-f0-9]{80}$").unwrap(),
                modes: HashInfo {
 modes: vec![
Mode {
        john: None,
                        hashcat: None,
                        extended: false,
                        name: "RIPEMD-320".to_string()
                    }
                ]
           }},
            Prototype {
        regex: Regex::new(r"(?i)^\$episerver\$\*1\*[a-z0-9/=+]+\*[a-z0-9/=+]{42,43}$").unwrap(),
                modes: HashInfo {
 modes: vec![
Mode {
        john: Some("episerver".to_string()),
                        hashcat: Some(1441),
                        extended: false,
                        name: "EPiServer 6.x \u{2265} v4".to_string()
                    }
                ]
           }},
            Prototype {
        regex: Regex::new(r"(?i)^0x0100[a-f0-9]{88}$").unwrap(),
                modes: HashInfo {
 modes: vec![
Mode {
        john: Some("mssql".to_string()),
                        hashcat: Some(131),
                        extended: false,
                        name: "MSSQL(2000)".to_string()
                    }
                ]
           }},
            Prototype {
        regex: Regex::new(r"(?i)^[a-f0-9]{96}$").unwrap(),
                modes: HashInfo {
 modes: vec![
Mode {
        john: Some("raw-sha384".to_string()),
                        hashcat: Some(10800),
                        extended: false,
                        name: "SHA-384".to_string()
                    },
                    Mode {
        john: None,
                        hashcat: None,
                        extended: false,
                        name: "SHA3-384".to_string()
                    },
                    Mode {
        john: None,
                        hashcat: None,
                        extended: false,
                        name: "Skein-512(384)".to_string()
                    },
                    Mode {
        john: None,
                        hashcat: None,
                        extended: false,
                        name: "Skein-1024(384)".to_string()
                    }
                ]
           }},
            Prototype {
        regex: Regex::new(r"^SSHA512[a-z0-9/+]{96}$").unwrap(),
                modes: HashInfo {
 modes: vec![
Mode {
        john: Some("ssha512".to_string()),
                        hashcat: Some(1711),
                        extended: false,
                        name: "SSHA-512(Base64)".to_string()
                    },
                    Mode {
        john: Some("ssha512".to_string()),
                        hashcat: Some(1711),
                        extended: false,
                        name: "LDAP(SSHA-512)".to_string()
                    }
                ]
           }},
            Prototype {
                regex: Regex::new(r"ssha512[0-9]{2}\$[a-z0-9A-Z/.]{16,48}\$[a-z0-9A-Z/.]{86}$").unwrap(),
                modes: HashInfo {
 modes: vec![
                    Mode {
                        john: Some("aix-ssha512".to_string()),
                        hashcat: Some(6500),
                        extended: false,
                        name: "AIX(ssha512)".to_string()
                    }
                ]
           }},
            Prototype {
                regex: Regex::new(r"(?i)^[a-f0-9]{128}(:.+)?$").unwrap(),
                modes: HashInfo {
 modes: vec![
                    Mode {
                        john: Some("raw-sha512".to_string()),
                        hashcat: Some(1700),
                        extended: false,
                        name: "SHA-512".to_string()
                    },
                    Mode {
                        john: Some("whirlpool".to_string()),
                        hashcat: Some(6100),
                        extended: false,
                        name: "Whirlpool".to_string()
                    },
                    Mode {
                        john: None,
                        hashcat: None,
                        extended: false,
                        name: "Salsa10".to_string()
                    },
                    Mode {
                        john: None,
                        hashcat: None,
                        extended: false,
                        name: "Salsa20".to_string()
                    },
                    Mode {
                        john: Some("raw-keccak".to_string()),
                        hashcat: None,
                        extended: false,
                        name: "SHA3-512".to_string()
                    },
                    Mode {
                        john: Some("skein-512".to_string()),
                        hashcat: None,
                        extended: false,
                        name: "Skein-512".to_string()
                    },
                    Mode {
                        john: None,
                        hashcat: None,
                        extended: false,
                        name: "Skein-1024(512)".to_string()
                    },
                    Mode {
                        john: None,
                        hashcat: Some(1710),
                        extended: true,
                        name: "sha512($pass.$salt)".to_string()
                    },
                    Mode {
                        john: None,
                        hashcat: Some(1720),
                        extended: true,
                        name: "sha512($salt.$pass)".to_string()
                    },
                    Mode {
                        john: None,
                        hashcat: Some(1730),
                        extended: true,
                        name: "sha512(unicode($pass).$salt)".to_string()
                    },
                    Mode {
                        john: None,
                        hashcat: Some(1740),
                        extended: true,
                        name: "sha512($salt.unicode($pass))".to_string()
                    },
                    Mode {
                        john: Some("hmac-sha512".to_string()),
                        hashcat: Some(1750),
                        extended: true,
                        name: "HMAC-SHA512 (key = $pass)".to_string()
                    },
                    Mode {
                        john: Some("hmac-sha512".to_string()),
                        hashcat: Some(1760),
                        extended: true,
                        name: "HMAC-SHA512 (key = $salt)".to_string()
                    }
                ]
           }},
            Prototype {
                regex: Regex::new(r"(?i)^[a-f0-9]{136}$").unwrap(),
                modes: HashInfo {
 modes: vec![
                    Mode {
                        john: Some("xsha512".to_string()),
                        hashcat: Some(1722),
                        extended: false,
                        name: "OSX v10.7".to_string()
                    }
                ]
           }},
            Prototype {
                regex: Regex::new(r"(?i)^0x0200[a-f0-9]{136}$").unwrap(),
                modes: HashInfo {
 modes: vec![
                    Mode {
                        john: Some("msql12".to_string()),
                        hashcat: Some(1731),
                        extended: false,
                        name: "MSSQL(2012)".to_string()
                    },
                    Mode {
                        john: Some("msql12".to_string()),
                        hashcat: Some(1731),
                        extended: false,
                        name: "MSSQL(2014)".to_string()
                    }
                ]
           }},
            Prototype {
                regex: Regex::new(r"(?i)^\$ml\$[0-9]+\$[a-f0-9]{64}\$[a-f0-9]{128}$").unwrap(),
                modes: HashInfo {
 modes: vec![
                    Mode {
                        john: Some("pbkdf2-hmac-sha512".to_string()),
                        hashcat: Some(7100),
                        extended: false,
                        name: "OSX v10.8".to_string()
                    },
                    Mode {
                        john: Some("pbkdf2-hmac-sha512".to_string()),
                        hashcat: Some(7100),
                        extended: false,
                        name: "OSX v10.9".to_string()
                    }
                ]
           }},
            Prototype {
                regex: Regex::new(r"(?i)^[a-f0-9]{256}$").unwrap(),
                modes: HashInfo {
 modes: vec![
                    Mode {
                        john: None,
                        hashcat: None,
                        extended: false,
                        name: "Skein-1024".to_string()
                    }
                ]
           }},
            Prototype {
                regex: Regex::new(r"(?i)^grub\.pbkdf2\.sha512\.[0-9]+\.([a-f0-9]{128,2048}\.|[0-9]+\.)?[a-f0-9]{128}$").unwrap(),
                modes: HashInfo {
 modes: vec![
                    Mode {
                        john: None,
                        hashcat: Some(7200),
                        extended: false,
                        name: "GRUB 2".to_string()
                    }
                ]
           }},
            Prototype {
                regex: Regex::new(r"(?i)^sha1\$[a-z0-9]+\$[a-f0-9]{40}$").unwrap(),
                modes: HashInfo {
 modes: vec![
                    Mode {
                        john: None,
                        hashcat: Some(124),
                        extended: false,
                        name: "Django(SHA-1)".to_string()
                    }
                ]
           }},
            Prototype {
                regex: Regex::new(r"(?i)^[a-f0-9]{49}$").unwrap(),
                modes: HashInfo {
 modes: vec![
                    Mode {
                        john: Some("citrix_ns10".to_string()),
                        hashcat: Some(8100),
                        extended: false,
                        name: "Citrix Netscaler".to_string()
                    }
                ]
           }},
            Prototype {
                regex: Regex::new(r"(?i)^\$S\$[a-z0-9/.]{52}$").unwrap(),
                modes: HashInfo {
 modes: vec![
                    Mode {
                        john: Some("drupal7".to_string()),
                        hashcat: Some(7900),
                        extended: false,
                        name: "Drupal > v7.x".to_string()
                    }
                ]
           }},
            Prototype {
                regex: Regex::new(r"(?i)^\$5\$(rounds=[0-9]+\$)?[a-z0-9/.]{0,16}\$[a-z0-9/.]{43}$").unwrap(),
                modes: HashInfo {
 modes: vec![
                    Mode {
                        john: Some("sha256crypt".to_string()),
                        hashcat: Some(7400),
                        extended: false,
                        name: "SHA-256 Crypt".to_string()
                    }
                ]
           }},
            Prototype {
                regex: Regex::new(r"(?i)^0x[a-f0-9]{4}[a-f0-9]{16}[a-f0-9]{64}$").unwrap(),
                modes: HashInfo {
 modes: vec![
                    Mode {
                        john: Some("sybasease".to_string()),
                        hashcat: Some(8000),
                        extended: false,
                        name: "Sybase ASE".to_string()
                    }
                ]
           }},
            Prototype {
                regex: Regex::new(r"(?i)^\$6\$(rounds=[0-9]+\$)?[a-z0-9/.]{0,16}\$[a-z0-9/.]{86}$").unwrap(),
                modes: HashInfo {
 modes: vec![
                    Mode {
                        john: Some("sha512crypt".to_string()),
                        hashcat: Some(1800),
                        extended: false,
                        name: "SHA-512 Crypt".to_string()
                    }
                ]
           }},
            Prototype {
                regex: Regex::new(r"(?i)^\$sha\$[a-z0-9]{1,16}\$([a-f0-9]{32}|[a-f0-9]{40}|[a-f0-9]{64}|[a-f0-9]{128}|[a-f0-9]{140})$").unwrap(),
                modes: HashInfo {
 modes: vec![
                    Mode {
                        john: None,
                        hashcat: None,
                        extended: false,
                        name: "Minecraft(AuthMe Reloaded)".to_string()
                    }
                ]
           }},
            Prototype {
                regex: Regex::new(r"(?i)^sha256\$[a-z0-9]+\$[a-f0-9]{64}$").unwrap(),
                modes: HashInfo {
 modes: vec![
                    Mode {
                        john: None,
                        hashcat: None,
                        extended: false,
                        name: "Django(SHA-256)".to_string()
                    }
                ]
           }},
            Prototype {
                regex: Regex::new(r"(?i)^sha384\$[a-z0-9]+\$[a-f0-9]{96}$").unwrap(),
                modes: HashInfo {
 modes: vec![
                    Mode {
                        john: None,
                        hashcat: None,
                        extended: false,
                        name: "Django(SHA-384)".to_string()
                    }
                ]
           }},
            Prototype {
                regex: Regex::new(r"(?i)^crypt1:[a-z0-9+=]{12}:[a-z0-9+=]{12}$").unwrap(),
                modes: HashInfo {
 modes: vec![
                    Mode {
                        john: None,
                        hashcat: None,
                        extended: false,
                        name: "Clavister Secure Gateway".to_string()
                    }
                ]
           }},
            Prototype {
                regex: Regex::new(r"(?i)^[a-f0-9]{112}$").unwrap(),
                modes: HashInfo {
 modes: vec![
                    Mode {
                        john: None,
                        hashcat: None,
                        extended: false,
                        name: "Cisco VPN Client(PCF-File)".to_string()
                    }
                ]
           }},
            Prototype {
                regex: Regex::new(r"(?i)^[a-f0-9]{1329}$").unwrap(),
                modes: HashInfo {
 modes: vec![
                    Mode {
                        john: None,
                        hashcat: None,
                        extended: false,
                        name: "Microsoft MSTSC(RDP-File)".to_string()
                    }
                ]
           }},
            Prototype {
                regex: Regex::new(r"(?i)^[^\\/:*?<>|]{1,20}[:]{2,3}([^\\/:*?<>|]{1,20})?:[a-f0-9]{48}:[a-f0-9]{48}:[a-f0-9]{16}$").unwrap(),
                modes: HashInfo {
 modes: vec![
                    Mode {
                        john: Some("netntlm".to_string()),
                        hashcat: Some(5500),
                        extended: false,
                        name: "NetNTLMv1-VANILLA / NetNTLMv1+ESS".to_string()
                    }
                ]
           }},
            Prototype {
                regex: Regex::new(r"(?i)^([^\\/:*?<>|]{1,20}\\)?[^\\/:*?<>|]{1,20}[:]{2,3}([^\\/:*?<>|]{1,20}:)?[^\\/:*?<>|]{1,20}:[a-f0-9]{32}:[a-f0-9]+$").unwrap(),
                modes: HashInfo {
 modes: vec![
                    Mode {
                        john: Some("netntlmv2".to_string()),
                        hashcat: Some(5600),
                        extended: false,
                        name: "NetNTLMv2".to_string()
                    }
                ]
           }},
            Prototype {
                regex: Regex::new(r"(?i)^\$(krb5pa|mskrb5)\$([0-9]{2})?\$.+\$[a-f0-9]+$").unwrap(),
                modes: HashInfo {
 modes: vec![
                    Mode {
                        john: Some("krb5pa-md5".to_string()),
                        hashcat: Some(7500),
                        extended: false,
                        name: "Kerberos 5 AS-REQ Pre-Auth".to_string()
                    }
                ]
           }},
            Prototype {
                regex: Regex::new(r"(?i)^\$scram\$[0-9]+\$[a-z0-9/.]{16}\$sha-1=[a-z0-9/.]{27},sha-256=[a-z0-9/.]{43},sha-512=[a-z0-9/.]{86}$").unwrap(),
                modes: HashInfo {
 modes: vec![
                    Mode {
                        john: None,
                        hashcat: None,
                        extended: false,
                        name: "SCRAM Hash".to_string()
                    }
                ]
           }},
            Prototype {
                regex: Regex::new(r"(?i)^[a-f0-9]{40}:[a-f0-9]{0,32}$").unwrap(),
                modes: HashInfo {
 modes: vec![
                    Mode {
                        john: None,
                        hashcat: Some(7600),
                        extended: false,
                        name: "Redmine Project Management Web App".to_string()
                    }
                ]
           }},
            Prototype {
                regex: Regex::new(r"(?i)^(.+)?\$[a-f0-9]{16}$").unwrap(),
                modes: HashInfo {
 modes: vec![
                    Mode {
                        john: Some("sapb".to_string()),
                        hashcat: Some(7700),
                        extended: false,
                        name: "SAP CODVN B (BCODE)".to_string()
                    }
                ]
           }},
            Prototype {
                regex: Regex::new(r"(?i)^(.+)?\$[a-f0-9]{40}$").unwrap(),
                modes: HashInfo {
 modes: vec![
                    Mode {
                        john: Some("sapg".to_string()),
                        hashcat: Some(7800),
                        extended: false,
                        name: "SAP CODVN F/G (PASSCODE)".to_string()
                    }
                ]
           }},
            Prototype {
                regex: Regex::new(r"(?i)^(.+\$)?[a-z0-9/.+]{30}(:.+)?$").unwrap(),
                modes: HashInfo {
 modes: vec![
                    Mode {
                        john: Some("md5ns".to_string()),
                        hashcat: Some(22),
                        extended: false,
                        name: "Juniper Netscreen/SSG(ScreenOS)".to_string()
                    }
                ]
           }},
            Prototype {
                regex: Regex::new(r"(?i)^0x[a-f0-9]{60}\s0x[a-f0-9]{40}$").unwrap(),
                modes: HashInfo {
 modes: vec![
                    Mode {
                        john: None,
                        hashcat: Some(123),
                        extended: false,
                        name: "EPi".to_string()
                    }
                ]
           }},
            Prototype {
                regex: Regex::new(r"(?i)^[a-f0-9]{40}:[^*]{1,25}$").unwrap(),
                modes: HashInfo {
 modes: vec![
                    Mode {
                        john: None,
                        hashcat: Some(121),
                        extended: false,
                        name: "SMF \u{2265} v1.1".to_string()
                    }
                ]
           }},
            Prototype {
                regex: Regex::new(r"(?i)^(\$wbb3\$\*1\*)?[a-f0-9]{40}[:*][a-f0-9]{40}$").unwrap(),
                modes: HashInfo {
 modes: vec![
                    Mode {
                        john: Some("wbb3".to_string()),
                        hashcat: Some(8400),
                        extended: false,
                        name: "Woltlab Burning Board 3.x".to_string()
                    }
                ]
           }},
            Prototype {
                regex: Regex::new(r"(?i)^[a-f0-9]{130}(:[a-f0-9]{40})?$").unwrap(),
                modes: HashInfo {
 modes: vec![
                    Mode {
                        john: None,
                        hashcat: Some(7300),
                        extended: false,
                        name: "IPMI2 RAKP HMAC-SHA1".to_string()
                    }
                ]
           }},
            Prototype {
                regex: Regex::new(r"(?i)^[a-f0-9]{32}:[0-9]+:[a-z0-9_.+-]+@[a-z0-9-]+\.[a-z0-9-.]+$").unwrap(),
                modes: HashInfo {
 modes: vec![
                    Mode {
                        john: None,
                        hashcat: Some(6800),
                        extended: false,
                        name: "Lastpass".to_string()
                    }
                ]
           }},
            Prototype {
                regex: Regex::new(r"(?i)^[a-z0-9/.]{16}([:$].+)?$").unwrap(),
                modes: HashInfo {
 modes: vec![
                    Mode {
                        john: Some("asa-md5".to_string()),
                        hashcat: Some(2410),
                        extended: false,
                        name: "Cisco-ASA(MD5)".to_string()
                    }
                ]
           }},
            Prototype {
                regex: Regex::new(r"(?i)^\$vnc\$\*[a-f0-9]{32}\*[a-f0-9]{32}$").unwrap(),
                modes: HashInfo {
 modes: vec![
                    Mode {
                        john: Some("vnc".to_string()),
                        hashcat: None,
                        extended: false,
                        name: "VNC".to_string()
                    }
                ]
           }},
            Prototype {
                regex: Regex::new(r"(?i)^[a-z0-9]{32}(:([a-z0-9-]+\.)?[a-z0-9-.]+\.[a-z]{2,7}:.+:[0-9]+)?$").unwrap(),
                modes: HashInfo {
 modes: vec![
                    Mode {
                        john: None,
                        hashcat: Some(8300),
                        extended: false,
                        name: "DNSSEC(NSEC3)".to_string()
                    }
                ]
           }},
            Prototype {
                regex: Regex::new(r"(?i)^(user-.+:)?\$racf\$\*.+\*[a-f0-9]{16}$").unwrap(),
                modes: HashInfo {
 modes: vec![
                    Mode {
                        john: Some("racf".to_string()),
                        hashcat: Some(8500),
                        extended: false,
                        name: "RACF".to_string()
                    }
                ]
           }},
            Prototype {
                regex: Regex::new(r"(?i)^\$3\$\$[a-f0-9]{32}$").unwrap(),
                modes: HashInfo {
 modes: vec![
                    Mode {
                        john: None,
                        hashcat: None,
                        extended: false,
                        name: "NTHash(FreeBSD Variant)".to_string()
                    }
                ]
           }},
            Prototype {
                regex: Regex::new(r"(?i)^\$sha1\$[0-9]+\$[a-z0-9/.]{0,64}\$[a-z0-9/.]{28}$").unwrap(),
                modes: HashInfo {
 modes: vec![
                    Mode {
                        john: Some("sha1crypt".to_string()),
                        hashcat: None,
                        extended: false,
                        name: "SHA-1 Crypt".to_string()
                    }
                ]
           }},
            Prototype {
                regex: Regex::new(r"(?i)^[a-f0-9]{70}$").unwrap(),
                modes: HashInfo {
 modes: vec![
                    Mode {
                        john: Some("hmailserver".to_string()),
                        hashcat: Some(1421),
                        extended: false,
                        name: "hMailServer".to_string()
                    }
                ]
           }},
            Prototype {
                regex: Regex::new(r"(?i)^[:$][AB][:$]([a-f0-9]{1,8}[:$])?[a-f0-9]{32}$").unwrap(),
                modes: HashInfo {
 modes: vec![
                    Mode {
                        john: Some("mediawiki".to_string()),
                        hashcat: Some(3711),
                        extended: false,
                        name: "MediaWiki".to_string()
                    }
                ]
           }},
            Prototype {
                regex: Regex::new(r"(?i)^[a-f0-9]{140}$").unwrap(),
                modes: HashInfo {
 modes: vec![
                    Mode {
                        john: None,
                        hashcat: None,
                        extended: false,
                        name: "Minecraft(xAuth)".to_string()
                    }
                ]
           }},
            Prototype {
                regex: Regex::new(r"(?i)^\$pbkdf2(-sha1)?\$[0-9]+\$[a-z0-9/.]+\$[a-z0-9/.]{27}$").unwrap(),
                modes: HashInfo {
 modes: vec![
                    Mode {
                        john: None,
                        hashcat: None,
                        extended: false,
                        name: "PBKDF2-SHA1(Generic)".to_string()
                    }
                ]
           }},
            Prototype {
                regex: Regex::new(r"(?i)^\$pbkdf2-sha256\$[0-9]+\$[a-z0-9/.]+\$[a-z0-9/.]{43}$").unwrap(),
                modes: HashInfo {
 modes: vec![
                    Mode {
                        john: Some("pbkdf2-hmac-sha256".to_string()),
                        hashcat: None,
                        extended: false,
                        name: "PBKDF2-SHA256(Generic)".to_string()
                    }
                ]
           }},
            Prototype {
                regex: Regex::new(r"(?i)^\$pbkdf2-sha512\$[0-9]+\$[a-z0-9/.]+\$[a-z0-9/.]{86}$").unwrap(),
                modes: HashInfo {
 modes: vec![
                    Mode {
                        john: None,
                        hashcat: None,
                        extended: false,
                        name: "PBKDF2-SHA512(Generic)".to_string()
                    }
                ]
           }},
            Prototype {
                regex: Regex::new(r"(?i)^\$p5k2\$[0-9]+\$[a-z0-9/+=-]+\$[a-z0-9/+-]{27}=$").unwrap(),
                modes: HashInfo {
 modes: vec![
                    Mode {
                        john: None,
                        hashcat: None,
                        extended: false,
                        name: "PBKDF2(Cryptacular)".to_string()
                    }
                ]
           }},
            Prototype {
                regex: Regex::new(r"(?i)^\$p5k2\$[0-9]+\$[a-z0-9/.]+\$[a-z0-9/.]{32}$").unwrap(),
                modes: HashInfo {
 modes: vec![
                    Mode {
                        john: None,
                        hashcat: None,
                        extended: false,
                        name: "PBKDF2(Dwayne Litzenberger)".to_string()
                    }
                ]
            }},
            Prototype {
                regex: Regex::new(r"^FSHP,[0123]\|[0-9]+\|[0-9]+}[a-z0-9A-Z/+=]+$").unwrap(),
                modes: HashInfo {
 modes: vec![
                    Mode {
                        john: None,
                        hashcat: None,
                        extended: false,
                        name: "Fairly Secure Hashed Password".to_string()
                    }
                ]
            }},
            Prototype {
                regex: Regex::new(r"(?i)^\$PHPS\$.+\$[a-f0-9]{32}$").unwrap(),
                modes: HashInfo {
 modes: vec![
                    Mode {
                        john: Some("phps".to_string()),
                        hashcat: Some(2612),
                        extended: false,
                        name: "PHPS".to_string()
                    }
                ]
            }},
            Prototype {
                regex: Regex::new(r"(?i)^[0-9]{4}:[a-f0-9]{16}:[a-f0-9]{2080}$").unwrap(),
                modes: HashInfo {
 modes: vec![
                    Mode {
                        john: None,
                        hashcat: Some(6600),
                        extended: false,
                        name: "1Password(Agile Keychain)".to_string()
                    }
                ]
            }},
            Prototype {
                regex: Regex::new(r"(?i)^[a-f0-9]{64}:[a-f0-9]{32}:[0-9]{5}:[a-f0-9]{608}$").unwrap(),
                modes: HashInfo {
 modes: vec![
                    Mode {
                        john: None,
                        hashcat: Some(8200),
                        extended: false,
                        name: "1Password(Cloud Keychain)".to_string()
                    }
                ]
            }},
            Prototype {
                regex: Regex::new(r"(?i)^[a-f0-9]{256}:[a-f0-9]{256}:[a-f0-9]{16}:[a-f0-9]{16}:[a-f0-9]{320}:[a-f0-9]{16}:[a-f0-9]{40}:[a-f0-9]{40}:[a-f0-9]{32}$").unwrap(),
                modes: HashInfo {
 modes: vec![
                    Mode {
                        john: None,
                        hashcat: Some(5300),
                        extended: false,
                        name: "IKE-PSK MD5".to_string()
                    }
                ]
            }},
            Prototype {
                regex: Regex::new(r"(?i)^[a-f0-9]{256}:[a-f0-9]{256}:[a-f0-9]{16}:[a-f0-9]{16}:[a-f0-9]{320}:[a-f0-9]{16}:[a-f0-9]{40}:[a-f0-9]{40}:[a-f0-9]{40}$").unwrap(),
                modes: HashInfo {
 modes: vec![
                    Mode {
                        john: None,
                        hashcat: Some(5400),
                        extended: false,
                        name: "IKE-PSK SHA1".to_string()
                    }
                ]
            }},
            Prototype {
                regex: Regex::new(r"(?i)^[a-z0-9/+]{27}=$").unwrap(),
                modes: HashInfo {
 modes: vec![
                    Mode {
                        john: None,
                        hashcat: Some(133),
                        extended: false,
                        name: "PeopleSoft".to_string()
                    }
                ]
            }},
            Prototype {
                regex: Regex::new(r"(?i)^crypt\$[a-f0-9]{5}\$[a-z0-9/.]{13}$").unwrap(),
                modes: HashInfo {
 modes: vec![
                    Mode {
                        john: None,
                        hashcat: None,
                        extended: false,
                        name: "Django(DES Crypt Wrapper)".to_string()
                    }
                ]
            }},
            Prototype {
                regex: Regex::new(r"(?i)^(\$django\$\*1\*)?pbkdf2_sha256\$[0-9]+\$[a-z0-9]+\$[a-z0-9/+=]{44}$").unwrap(),
                modes: HashInfo {
 modes: vec![
                    Mode {
                        john: Some("django".to_string()),
                        hashcat: Some(10000),
                        extended: false,
                        name: "Django(PBKDF2-HMAC-SHA256)".to_string()
                    }
                ]
            }},
            Prototype {
                regex: Regex::new(r"(?i)^pbkdf2_sha1\$[0-9]+\$[a-z0-9]+\$[a-z0-9/+=]{28}$").unwrap(),
                modes: HashInfo {
 modes: vec![
                    Mode {
                        john: None,
                        hashcat: None,
                        extended: false,
                        name: "Django(PBKDF2-HMAC-SHA1)".to_string()
                    }
                ]
            }},
            Prototype {
                regex: Regex::new(r"(?i)^bcrypt(\$2[axy]|\$2)\$[0-9]{2}\$[a-z0-9/.]{53}$").unwrap(),
                modes: HashInfo {
 modes: vec![
                    Mode {
                        john: None,
                        hashcat: None,
                        extended: false,
                        name: "Django(bcrypt)".to_string()
                    }
                ]
            }},
            Prototype {
                regex: Regex::new(r"(?i)^md5\$[a-f0-9]+\$[a-f0-9]{32}$").unwrap(),
                modes: HashInfo {
 modes: vec![
                    Mode {
                        john: None,
                        hashcat: None,
                        extended: false,
                        name: "Django(MD5)".to_string()
                    }
                ]
            }},
            Prototype {
                regex: Regex::new(r"(?i)^\{PKCS5S2}[a-z0-9/+]{64}$").unwrap(),
                modes: HashInfo {
 modes: vec![
                    Mode {
                        john: None,
                        hashcat: None,
                        extended: false,
                        name: "PBKDF2(Atlassian)".to_string()
                    }
                ]
            }},
            Prototype {
                regex: Regex::new(r"(?i)^md5[a-f0-9]{32}$").unwrap(),
                modes: HashInfo {
 modes: vec![
                    Mode {
                        john: None,
                        hashcat: None,
                        extended: false,
                        name: "PostgreSQL MD5".to_string()
                    }
                ]
            }},
            Prototype {
                regex: Regex::new(r"(?i)^\([a-z0-9/+]{49}\)$").unwrap(),
                modes: HashInfo {
 modes: vec![
                    Mode {
                        john: None,
                        hashcat: Some(9100),
                        extended: false,
                        name: "Lotus Notes/Domino 8".to_string()
                    }
                ]
            }},
            Prototype {
                regex: Regex::new(r"(?i)^SCRYPT:[0-9]+:[0-9]:[0-9]:[a-z0-9:/+=]+$").unwrap(),
                modes: HashInfo {
 modes: vec![
                    Mode {
                        john: None,
                        hashcat: Some(8900),
                        extended: false,
                        name: "scrypt".to_string()
                    }
                ]
            }},
            Prototype {
                regex: Regex::new(r"(?i)^\$8\$[a-z0-9/.]{14}\$[a-z0-9/.]{43}$").unwrap(),
                modes: HashInfo {
 modes: vec![
                    Mode {
                        john: Some("cisco8".to_string()),
                        hashcat: Some(9200),
                        extended: false,
                        name: "Cisco Type 8".to_string()
                    }
                ]
            }},
            Prototype {
                regex: Regex::new(r"(?i)^\$9\$[a-z0-9/.]{14}\$[a-z0-9/.]{43}$").unwrap(),
                modes: HashInfo {
 modes: vec![
                    Mode {
                        john: Some("cisco9".to_string()),
                        hashcat: Some(9300),
                        extended: false,
                        name: "Cisco Type 9".to_string()
                    }
                ]
            }},
            Prototype {
                regex: Regex::new(r"(?i)^\$office\$\*2007\*[0-9]{2}\*[0-9]{3}\*[0-9]{2}\*[a-z0-9]{32}\*[a-z0-9]{32}\*[a-z0-9]{40}$").unwrap(),
                modes: HashInfo {
 modes: vec![
                    Mode {
                        john: Some("office".to_string()),
                        hashcat: Some(9400),
                        extended: false,
                        name: "Microsoft Office 2007".to_string()
                    }
                ]
            }},
            Prototype {
                regex: Regex::new(r"(?i)^\$office\$\*2010\*[0-9]{6}\*[0-9]{3}\*[0-9]{2}\*[a-z0-9]{32}\*[a-z0-9]{32}\*[a-z0-9]{64}$").unwrap(),
                modes: HashInfo {
 modes: vec![
                    Mode {
                        john: None,
                        hashcat: Some(9500),
                        extended: false,
                        name: "Microsoft Office 2010".to_string()
                    }
                ]
            }},
            Prototype {
                regex: Regex::new(r"(?i)^\$office\$\*2013\*[0-9]{6}\*[0-9]{3}\*[0-9]{2}\*[a-z0-9]{32}\*[a-z0-9]{32}\*[a-z0-9]{64}$").unwrap(),
                modes: HashInfo {
 modes: vec![
                        Mode {
                        john: None,
                        hashcat: Some(9600),
                        extended: false,
                        name: "Microsoft Office 2013".to_string()
                    }
                ]
            }},
            Prototype {
                regex: Regex::new(r"(?i)^\$fde\$[0-9]{2}\$[a-f0-9]{32}\$[0-9]{2}\$[a-f0-9]{32}\$[a-f0-9]{3072}$").unwrap(),
                modes: HashInfo {
 modes: vec![
                    Mode {
                        john: Some("fde".to_string()),
                        hashcat: Some(8800),
                        extended: false,
                        name: "Android FDE \u{2264} 4.3".to_string()
                    }
                ]
            }},
            Prototype {
                regex: Regex::new(r"(?i)^\$oldoffice\$[01]\*[a-f0-9]{32}\*[a-f0-9]{32}\*[a-f0-9]{32}$").unwrap(),
                modes: HashInfo {
                 modes: vec![
                    Mode {
                        john: Some("oldoffice".to_string()),
                        hashcat: Some(9700),
                        extended: false,
                        name: "Microsoft Office \u{2264} 2003 (MD5+RC4)".to_string()
                    },
                    Mode {
                        john: Some("oldoffice".to_string()),
                        hashcat: Some(9710),
                        extended: false,
                        name: "Microsoft Office \u{2264} 2003 (MD5+RC4) collider-mode #1".to_string()
                    },
                    Mode {
                        john: Some("oldoffice".to_string()),
                        hashcat: Some(9720),
                        extended: false,
                        name: "Microsoft Office \u{2264} 2003 (MD5+RC4) collider-mode #2".to_string()
                    }
                ]
            }},
            Prototype {
                regex: Regex::new(r"(?i)^\$oldoffice\$[34]\*[a-f0-9]{32}\*[a-f0-9]{32}\*[a-f0-9]{40}$").unwrap(),
                modes: HashInfo {
                 modes: vec![
                    Mode {
                        john: None,
                        hashcat: Some(9800),
                        extended: false,
                        name: "Microsoft Office \u{2264} 2003 (SHA1+RC4)".to_string()
                    },
                    Mode {
                        john: None,
                        hashcat: Some(9810),
                        extended: false,
                        name: "Microsoft Office \u{2264} 2003 (SHA1+RC4) collider-mode #1".to_string()
                    },
                    Mode {
                        john: None,
                        hashcat: Some(9820),
                        extended: false,
                        name: "Microsoft Office \u{2264} 2003 (SHA1+RC4) collider-mode #2".to_string()
                    }
                ]
            }},
            Prototype {
                regex: Regex::new(r"(?i)^(\$radmin2\$)?[a-f0-9]{32}$").unwrap(),
                modes: HashInfo {
                 modes: vec![
                    Mode {
                        john: Some("radmin".to_string()),
                        hashcat: Some(9900),
                        extended: false,
                        name: "RAdmin v2.x".to_string()
                    }
                ]
            }},
            Prototype {
                regex: Regex::new(r"^x-issha,\s[0-9]{4}[a-z0-9A-Z/+=]+$").unwrap(),
                modes: HashInfo {
                 modes: vec![
                    Mode {
                        john: Some("saph".to_string()),
                        hashcat: Some(10300),
                        extended: false,
                        name: "SAP CODVN H (PWDSALTEDHASH) iSSHA-1".to_string()
                    }
                ]
            }},
            Prototype {
                regex: Regex::new(r"(?i)^\$cram_md5\$[a-z0-9/+=-]+\$[a-z0-9/+=-]{52}$").unwrap(),
                modes: HashInfo {
                 modes: vec![
                    Mode {
                        john: None,
                        hashcat: Some(10200),
                        extended: false,
                        name: "CRAM-MD5".to_string()
                    }
                ]
            }},
            Prototype {
                regex: Regex::new(r"(?i)^[a-f0-9]{16}:2:4:[a-f0-9]{32}$").unwrap(),
                modes: HashInfo {
                 modes: vec![
                    Mode {
                        john: None,
                        hashcat: Some(10100),
                        extended: false,
                        name: "SipHash".to_string()
                    }
                ]
            }},
            Prototype {
                regex: Regex::new(r"(?i)^[a-f0-9]{4,}$").unwrap(),
                modes: HashInfo {
                 modes: vec![
                    Mode {
                        john: None,
                        hashcat: None,
                        extended: true,
                        name: "Cisco Type 7".to_string()
                    }
                ]
            }},
            Prototype {
                regex: Regex::new(r"(?i)^[a-z0-9/.]{13,}$").unwrap(),
                modes: HashInfo {
                 modes: vec![
                    Mode {
                        john: Some("bigcrypt".to_string()),
                        hashcat: None,
                        extended: true,
                        name: "BigCrypt".to_string()
                    }
                ]
            }},
            Prototype {
                regex: Regex::new(r"(?i)^(\$cisco4\$)?[a-z0-9/.]{43}$").unwrap(),
                modes: HashInfo {
                 modes: vec![
                    Mode {
                        john: Some("cisco4".to_string()),
                        hashcat: None,
                        extended: false,
                        name: "Cisco Type 4".to_string()
                    }
                ]
            }},
            Prototype {
                regex: Regex::new(r"(?i)^bcrypt_sha256\$\$(2[axy]|2)\$[0-9]+\$[a-z0-9/.]{53}$").unwrap(),
                modes: HashInfo {
                 modes: vec![
                    Mode {
                        john: None,
                        hashcat: None,
                        extended: false,
                        name: "Django(bcrypt-SHA256)".to_string()
                    }
                ]
            }},
            Prototype {
                regex: Regex::new(r"(?i)^\$postgres\$.[^*]+[*:][a-f0-9]{1,32}[*:][a-f0-9]{32}$").unwrap(),
                modes: HashInfo {
                 modes: vec![
                    Mode {
                        john: Some("postgres".to_string()),
                        hashcat: Some(11100),
                        extended: false,
                        name: "PostgreSQL Challenge-Response Authentication (MD5)".to_string()
                    }
                ]
            }},
            Prototype {
                regex: Regex::new(r"(?i)^\$siemens-s7\$[0-9]\$[a-f0-9]{40}\$[a-f0-9]{40}$").unwrap(),
                modes: HashInfo {
                 modes: vec![
                    Mode {
                        john: Some("siemens-s7".to_string()),
                        hashcat: None,
                        extended: false,
                        name: "Siemens-S7".to_string()
                    }
                ]
            }},
            Prototype {
                regex: Regex::new(r"(?i)^(\$pst\$)?[a-f0-9]{8}$").unwrap(),
                modes: HashInfo {
                 modes: vec![
                    Mode {
                        john: None,
                        hashcat: None,
                        extended: false,
                        name: "Microsoft Outlook PST".to_string()
                    }
                ]
            }},
            Prototype {
                regex: Regex::new(r"(?i)^sha256[:$][0-9]+[:$][a-z0-9/+]+[:$][a-z0-9/+]{32,128}$").unwrap(),
                modes: HashInfo {
                 modes: vec![
                    Mode {
                        john: None,
                        hashcat: Some(10900),
                        extended: false,
                        name: "PBKDF2-HMAC-SHA256(PHP)".to_string()
                    }
                ]
            }},
            Prototype {
                regex: Regex::new(r"(?i)^(\$dahua\$)?[a-z0-9]{8}$").unwrap(),
                modes: HashInfo {
                 modes: vec![
                    Mode {
                        john: Some("dahua".to_string()),
                        hashcat: None,
                        extended: false,
                        name: "Dahua".to_string()
                    }
                ]
            }},
            Prototype {
                regex: Regex::new(r"(?i)^\$mysqlna\$[a-f0-9]{40}[:*][a-f0-9]{40}$").unwrap(),
                modes: HashInfo {
                 modes: vec![
                    Mode {
                        john: None,
                        hashcat: Some(11200),
                        extended: false,
                        name: "MySQL Challenge-Response Authentication (SHA1)".to_string()
                    }
                ]
            }},
            Prototype {
                regex: Regex::new(r"(?i)^\$pdf\$[24]\*[34]\*128\*[0-9-]{1,5}\*1\*(16|32)\*[a-f0-9]{32,64}\*32\*[a-f0-9]{64}\*(8|16|32)\*[a-f0-9]{16,64}$").unwrap(),
                modes: HashInfo {
                 modes: vec![
                    Mode {
                        john: Some("pdf".to_string()),
                        hashcat: Some(10500),
                        extended: false,
                        name: "PDF 1.4 - 1.6 (Acrobat 5 - 8)".to_string()
                    }
                ]
            }},
    ];
}
