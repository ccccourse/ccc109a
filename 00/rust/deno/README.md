# Build Deno

* https://deno.land/manual/contributing/building_from_source

## 系統架構 Architecture

* https://deno.land/manual/contributing/architecture

![](https://deno.land/images/schematic_v0.2.png)

## 源碼結構

* https://github.com/denoland/deno
    * core -- 核心
    * std -- 標準函式庫
    * cli -- 面對使用者的部分
    * docs -- 說明文件
    * tools -- 工具程式 (*.py, ....)

## 結果:成功

```
user@DESKTOP-96FRN6B MINGW64 /d/ccc/test
$ git clone --recursive https://github.com/denoland/deno.git
Cloning into 'deno'...
remote: Enumerating objects: 36, done.
remote: Counting objects: 100% (36/36), done.
remote: Compressing objects: 100% (31/31), done.
remote: Total 32099 (delta 14), reused 11 (delta 5), pack-reused 32063 eceiving objects: 100% (32099/32099), 1Receiving objects: 100% (32099/32099), 15.57 MiB | 2.50 MiB/s, done.

Resolving deltas: 100% (23930/23930), done.
Submodule 'typescript' (https://github.com/microsoft/TypeScript.git) registered for path 'deno_typescript/typescript'
Submodule 'deno_third_party' (https://github.com/denoland/deno_third_party.git) registered for path 'third_party'
Cloning into 'D:/ccc/test/deno/deno_typescript/typescript'...
remote: Enumerating objects: 54434, done.        
remote: Counting objects: 100% (54434/54434), done.        
remote: Compressing objects: 100% (43662/43662), done.        
remote: Total 54434 (delta 10271), reused 32478 (delta 9101), pack-reused 0
Receiving objects: 100% (54434/54434), 28.56 MiB | 1.16 MiB/s, done.
Resolving deltas: 100% (10271/10271), done.
Cloning into 'D:/ccc/test/deno/third_party'...
remote: Enumerating objects: 14, done.        
remote: Counting objects: 100% (14/14), done.        
remote: Compressing objects: 100% (11/11), done.        
remote: Total 111166 (delta 5), reused 5 (delta 3), pack-reused 111152        
Receiving objects: 100% (111166/111166), 275.51 MiB | 2.22 MiB/s, done.
Resolving deltas: 100% (54972/54972), done.
remote: Enumerating objects: 397918, done.
remote: Counting objects: 100% (397899/397899), done.
remote: Compressing objects: 100% (104285/104285), done.
remote: Total 381439 (delta 277659), reused 365004 (delta 262302), pack-reused 0
Receiving objects: 100% (381439/381439), 804.24 MiB | 2.33 MiB/s, done.
Resolving deltas: 100% (277659/277659), completed with 12936 local objects.
From https://github.com/microsoft/TypeScript
 * branch                  551f0dd9a1b57ecd527a665b0af7fc98cd107af6 -> FETCH_HEAD
Submodule path 'deno_typescript/typescript': checked out '551f0dd9a1b57ecd527a665b0af7fc98cd107af6'
Submodule path 'third_party': checked out '9ad53352a9bc7cd179d9e06663a097352514d389'

user@DESKTOP-96FRN6B MINGW64 /d/ccc/test
$ cd debi
bash: cd: debi: No such file or directory

user@DESKTOP-96FRN6B MINGW64 /d/ccc/test
$ cd deno

user@DESKTOP-96FRN6B MINGW64 /d/ccc/test/deno (master)
$ cargo build
   Compiling cfg-if v0.1.10
   Compiling winapi v0.3.8
   Compiling proc-macro2 v1.0.17
   Compiling unicode-xid v0.2.0
   Compiling syn v1.0.29
   Compiling libc v0.2.71
   Compiling log v0.4.8                                                                                       
   Compiling memchr v2.3.3
   Compiling autocfg v1.0.0
   Compiling lazy_static v1.4.0                                                                               
   Compiling byteorder v1.3.4
   Compiling getrandom v0.1.14
   Compiling proc-macro-hack v0.5.16
   Compiling itoa v0.4.5
   Compiling fnv v1.0.7
   Compiling rand_core v0.4.2
   Compiling autocfg v0.1.7
   Compiling iovec v0.1.4
   Compiling serde v1.0.111                                                                                   
   Compiling matches v0.1.8
   Compiling smallvec v1.4.0
   Compiling slab v0.4.2
   Compiling futures-core v0.3.5
   Compiling winapi-build v0.1.1                                                                              
   Compiling once_cell v1.4.0
   Compiling futures-sink v0.3.5
   Compiling ppv-lite86 v0.2.8
   Compiling bytes v0.5.4
   Compiling proc-macro-nested v0.1.4
   Compiling winapi v0.2.8
   Compiling futures v0.1.29                                                                                  
   Compiling futures-io v0.3.5
   Compiling pin-utils v0.1.0
   Compiling maybe-uninit v2.0.0                                                                              
   Compiling ryu v1.0.4                                                                                       
   Compiling cc v1.0.54
   Compiling siphasher v0.3.3                                                                                 
   Compiling stable_deref_trait v1.1.1
   Compiling version_check v0.1.5                                                                             
   Compiling bitflags v1.2.1                                                                                  
   Compiling gimli v0.21.0
   Compiling percent-encoding v2.1.0
   Compiling semver-parser v0.7.0
   Compiling rustc-demangle v0.1.16
   Compiling object v0.19.0
   Compiling pin-project-lite v0.1.5
   Compiling httparse v1.3.4
   Compiling typenum v1.12.0                                                                                  
   Compiling regex-syntax v0.6.18
   Compiling siphasher v0.2.3
   Compiling strsim v0.9.3
   Compiling ident_case v1.0.1                                                                                
   Compiling untrusted v0.7.1                                                                                 
   Compiling unicode-width v0.1.7                                                                             
   Compiling spin v0.5.2
   Compiling version_check v0.9.2                                                                             
   Compiling cargo_gn v0.0.15
   Compiling base64 v0.11.0
   Compiling scopeguard v0.3.3                                                                                
   Compiling precomputed-hash v0.1.1
   Compiling new_debug_unreachable v1.0.4                                                                     
   Compiling percent-encoding v1.0.1
   Compiling byte-tools v0.3.1
   Compiling try-lock v0.2.2
   Compiling scoped-tls v1.0.0
   Compiling if_chain v0.1.3
   Compiling proc-macro2 v0.4.30
   Compiling opaque-debug v0.2.3
   Compiling fake-simd v0.1.2
   Compiling unicode-xid v0.1.0
   Compiling downcast-rs v1.1.1                                                                               
   Compiling alloc-no-stdlib v2.0.1                                                                           
   Compiling either v1.5.3                                                                                    
   Compiling tower-service v0.3.0
   Compiling crc32fast v1.2.0
   Compiling mime v0.3.16
   Compiling adler32 v1.0.4
   Compiling syn v0.15.44
   Compiling encoding_rs v0.8.23
   Compiling dtoa v0.4.5                                                                                      
   Compiling base64 v0.12.1
   Compiling safemem v0.3.3                                                                                   
   Compiling utf-8 v0.7.5
   Compiling quick-error v1.2.3
   Compiling anymap v0.12.1
   Compiling scopeguard v1.1.0
   Compiling strsim v0.8.0
   Compiling unicode-segmentation v1.6.0
   Compiling vec_map v0.8.2
   Compiling urlencoding v1.0.0
   Compiling glob v0.3.0
   Compiling semver-parser v0.9.0
   Compiling thread_local v1.0.1                                                                              
   Compiling indexmap v1.3.2
   Compiling num-traits v0.2.11
   Compiling num-integer v0.1.42
   Compiling num-bigint v0.2.6
   Compiling crossbeam-utils v0.7.2
   Compiling rand_core v0.3.1                                                                                 
   Compiling unicode-bidi v0.3.4
   Compiling unicode-normalization v0.1.12
   Compiling rand_pcg v0.1.2
   Compiling rand_chacha v0.1.1
   Compiling rand v0.6.5                                                                                      
   Compiling futures-channel v0.3.5
   Compiling futures-task v0.3.5
   Compiling kernel32-sys v0.2.2                                                                              
   Compiling ws2_32-sys v0.2.1
   Compiling http v0.2.1
   Compiling input_buffer v0.3.1                                                                              
   Compiling phf_shared v0.8.0
   Compiling owning_ref v0.4.1
   Compiling owning_ref v0.3.3                                                                                
   Compiling unicase v1.4.2
   Compiling semver v0.9.0
   Compiling ring v0.16.13
   Compiling sys-info v0.6.1
   Compiling addr2line v0.12.1
   Compiling textwrap v0.11.0
   Compiling unicase v2.6.0
   Compiling block-padding v0.1.5
   Compiling alloc-stdlib v0.2.1
   Compiling miniz_oxide v0.3.6
   Compiling rand_isaac v0.1.1                                                                                
   Compiling rand_xorshift v0.1.1                                                                             
   Compiling rand_hc v0.1.0
   Compiling idna v0.2.0                                                                                      
   Compiling idna v0.1.5
   Compiling lock_api v0.1.5
   Compiling rustc_version v0.2.3
   Compiling http-body v0.3.1
   Compiling headers-core v0.2.0
   Compiling brotli-decompressor v2.3.1
   Compiling num_cpus v1.13.0
   Compiling backtrace v0.3.48
   Compiling quote v1.0.6
   Compiling aho-corasick v0.7.10
   Compiling twoway v0.1.8                                                                                    
   Compiling buf_redux v0.8.4
   Compiling want v0.3.0                                                                                      
   Compiling log v0.3.9
   Compiling rand_core v0.5.1                                                                                 
   Compiling const-random-macro v0.1.8
   Compiling bytes v0.4.12                                                                                    
   Compiling base64 v0.10.1
   Compiling fxhash v0.2.1
   Compiling smallvec v0.6.13
   Compiling url v2.1.1
   Compiling url v1.7.2
   Compiling parking_lot_core v0.4.0
   Compiling generic-array v0.12.3
   Compiling flate2 v1.0.14
   Compiling rand_os v0.1.3
   Compiling rand_jitter v0.1.4
   Compiling net2 v0.2.34
   Compiling socket2 v0.3.12                                                                                  
   Compiling winapi-util v0.1.5
   Compiling atty v0.2.14
   Compiling time v0.1.43
   Compiling rand v0.4.6                                                                                      
   Compiling remove_dir_all v0.5.2
   Compiling dirs-sys v0.3.4
   Compiling filetime v0.2.10                                                                                 
   Compiling winreg v0.7.0
   Compiling quote v0.6.13
   Compiling brotli v3.3.0
   Compiling failure v0.1.8
   Compiling mime v0.2.6
   Compiling regex v1.3.9
   Compiling rand_pcg v0.2.1
   Compiling rand_chacha v0.2.2
   Compiling const-random v0.1.8
   Compiling tokio-io v0.1.13                                                                                 
   Compiling crossbeam-channel v0.4.2
   Compiling utime v0.3.0
   Compiling phf_shared v0.7.24
   Compiling digest v0.8.1
   Compiling block-buffer v0.7.3
   Compiling mime_guess v2.0.3
   Compiling miow v0.2.1                                                                                      
   Compiling miow v0.3.4                                                                                      
   Compiling termcolor v1.1.0                                                                                 
   Compiling same-file v1.0.6                                                                                 
   Compiling clap v2.33.1
   Compiling parking_lot_core v0.2.14
   Compiling dirs v2.0.2
   Compiling webpki v0.21.2                                                                                   
   Compiling sct v0.6.0
   Compiling which v3.1.1
   Compiling rand v0.7.3
   Compiling Inflector v0.11.4
   Compiling ahash v0.3.5
   Compiling async-compression v0.3.4                                                                         
   Compiling pmutil v0.5.3
   Compiling darling_core v0.10.2                                                                             
   Compiling phf v0.7.24
   Compiling sha-1 v0.8.2
   Compiling phf_generator v0.7.24                                                                            
   Compiling mio v0.6.22
   Compiling fwdansi v1.1.0
   Compiling walkdir v2.3.1
   Compiling rustyline v6.1.2
   Compiling parking_lot v0.4.8
   Compiling webpki-roots v0.19.0
   Compiling rustls v0.17.0
   Compiling rusty_v8 v0.4.2
   Compiling serde_derive v1.0.111                                                                            
   Compiling pin-project-internal v0.4.17
   Compiling futures-macro v0.3.5                                                                             
   Compiling tokio-macros v0.2.5
   Compiling phf_generator v0.8.0
   Compiling tempfile v3.1.0
   Compiling uuid v0.8.1
   Compiling dashmap v3.5.1
   Compiling swc_macros_common v0.3.1
   Compiling tungstenite v0.10.1
   Compiling headers v0.3.2
   Compiling phf_codegen v0.7.24
   Compiling parking_lot v0.7.1
   Compiling dlopen_derive v0.1.4                                                                             
   Compiling mio-named-pipes v0.1.6
   Compiling chashmap v2.2.2
   Compiling darling_macro v0.10.2
   Compiling string_cache_codegen v0.5.1
   Compiling pin-project v0.4.17                                                                              
   Compiling from_variant v0.1.2
   Compiling enum_kind v0.2.0
   Compiling string_enum v0.3.0                                                                               
   Compiling swc_ecma_parser_macros v0.4.1                                                                    
   Compiling swc_ecma_visit_macros v0.4.0
   Compiling mime_guess v1.8.8
   Compiling tokio v0.2.21
   Compiling notify v5.0.0-pre.2
   Compiling dlopen v0.1.8
   Compiling futures-util v0.3.5
   Compiling darling v0.10.2
   Compiling swc_atoms v0.2.2
   Compiling serde_json v1.0.53
   Compiling string_cache v0.8.0                                                                              
   Compiling serde_urlencoded v0.6.1                                                                          
   Compiling dprint-core v0.20.0
   Compiling ast_node v0.6.0
   Compiling sourcemap v5.0.0                                                                                 
   Compiling tokio-util v0.3.1
   Compiling tokio-rustls v0.13.1
   Compiling futures-executor v0.3.5
   Compiling h2 v0.2.5
   Compiling multipart v0.16.1                                                                                
   Compiling futures v0.3.5
   Compiling deno_core v0.46.0 (D:\ccc\test\deno\core)
   Compiling tokio-tungstenite v0.10.1
   Compiling deno_typescript v0.46.0 (D:\ccc\test\deno\deno_typescript)
   Compiling hyper v0.13.6
   Compiling swc_common v0.5.12
   Compiling deno v1.0.3 (D:\ccc\test\deno\cli)
   Compiling test_plugin v0.0.1 (D:\ccc\test\deno\test_plugin)
   Compiling hyper-rustls v0.20.0
   Compiling warp v0.2.3
   Compiling swc_ecma_ast v0.20.0
   Compiling hyper_hello v0.0.1 (D:\ccc\test\deno\tools\hyper_hello)
   Compiling reqwest v0.10.6
   Compiling swc_ecma_parser v0.23.1
   Compiling swc_ecma_visit v0.5.0
   Compiling dprint-plugin-typescript v0.19.0
    Finished dev [unoptimized + debuginfo] target(s) in 34m 48s

user@DESKTOP-96FRN6B MINGW64 /d/ccc/test/deno (master)
$ ls
Cargo.lock  cli   deno_typescript  LICENSE    Releases.md  target       third_party
Cargo.toml  core  docs             README.md  std          test_plugin  tools

```

## 建置後的執行檔

```
user@DESKTOP-96FRN6B MINGW64 /d/ccc/test/deno/target/debug (master)
$ ls
build     deps           hyper_hello.exe  libdeno_core.rlib        test_plugin.dll    
deno.d    examples       hyper_hello.pdb  libdeno_typescript.d     test_plugin.dll.d  
deno.exe  gn_out         incremental      libdeno_typescript.rlib  test_plugin.dll.lib
deno.pdb  hyper_hello.d  libdeno_core.d   test_plugin.d
```

執行

```
user@DESKTOP-96FRN6B MINGW64 /d/ccc/test/deno/target/debug (master)
$ ./deno run https://deno.land/std/examples/welcome.ts
Compile https://deno.land/std/examples/welcome.ts
Welcome to Deno �🦕
```
