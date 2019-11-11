
# step to produce the bug

1. git clone this repo
2. git submodule init 
3. git submodule update
4. cd vcpkg
5. ./bootstrap-vcpkg.sh
6. ./vcpkg install libsodium
7. cd ..
8. cargo build 
9. cargo run --bin demo  [there should be core dump :(]

```
$ gdb ./target/debug/demo core

in gdb command: layout asm

  >│0x562de9e81897 <crypto_scalarmult_curve25519_ref10_base+71>     vmovdqu (%rsi),%xmm0│
```



related information:
```
$ cat /etc/os-release

NAME="Ubuntu"
VERSION="18.04.3 LTS (Bionic Beaver)"
ID=ubuntu
ID_LIKE=debian
PRETTY_NAME="Ubuntu 18.04.3 LTS"
VERSION_ID="18.04"
HOME_URL="https://www.ubuntu.com/"
SUPPORT_URL="https://help.ubuntu.com/"
BUG_REPORT_URL="https://bugs.launchpad.net/ubuntu/"
PRIVACY_POLICY_URL="https://www.ubuntu.com/legal/terms-and-policies/privacy-policy"
VERSION_CODENAME=bionic
UBUNTU_CODENAME=bionic
```

```
$ gcc --version
gcc (Ubuntu 7.4.0-1ubuntu1~18.04.1) 7.4.0
Copyright (C) 2017 Free Software Foundation, Inc.
This is free software; see the source for copying conditions.  There is NO
warranty; not even for MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
```


```
$ rustc --version  # [latest stable release]
rustc 1.39.0 (4560ea788 2019-11-04)
```

```
$ cat /proc/cpuinfo

processor	: 19
vendor_id	: GenuineIntel
cpu family	: 15
model		: 6
model name	: Common KVM processor
stepping	: 1
microcode	: 0x1
cpu MHz		: 2297.338
cache size	: 16384 KB
physical id	: 1
siblings	: 10
core id		: 9
cpu cores	: 10
apicid		: 25
initial apicid	: 25
fpu		: yes
fpu_exception	: yes
cpuid level	: 13
wp		: yes
flags		: fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush mmx fxsr sse sse2 ht syscall nx lm constant_tsc nopl xtopology cpuid tsc_known_freq pni cx16 x2apic hypervisor lahf_lm cpuid_fault pti
bugs		: cpu_meltdown spectre_v1 spectre_v2 spec_store_bypass l1tf mds swapgs
bogomips	: 4594.67
clflush size	: 64
cache_alignment	: 128
address sizes	: 40 bits physical, 48 bits virtual
power management:
```