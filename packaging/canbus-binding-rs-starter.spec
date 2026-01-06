%global debug_package %{nil}

Name: canbus-binding-rs-starter
Version: 0.0.3+20251202+9+ga7ab3ad
Release: 1%{?dist}
Summary: canbus binding rust starter

License: GPL-3.0-or-later
Source0: %{name}-%{version}.tar.gz
Source1: vendor.tar.gz
Source2: cargo_config
Source3: starter.dbc

BuildRequires:   rust >= 1.70
BuildRequires:   cargo >= 1.70
BuildRequires: afb-librust
BuildRequires: clang-devel
BuildRequires:  clang
BuildRequires:  pkgconfig(json-c)
BuildRequires:  pkgconfig(afb-binding)
BuildRequires:  iputils
BuildRequires: clang-devel
BuildRequires: glibc-devel

%description
linky binding.

%prep
%autosetup -a1
mkdir -p .cargo
cp %{SOURCE2} .cargo/config.toml

%build
cargo build --offline --release --target %{_arch}-unknown-linux-gnu

%install
mkdir -p %{buildroot}%{_prefix}/redpesk/%{name}/lib
cp ./target/%{_arch}-unknown-linux-gnu/release/*.so %{buildroot}%{_prefix}/redpesk/%{name}/lib

mkdir -p %{buildroot}%{_prefix}/redpesk/%{name}/.rpconfig
cp %{SOURCE10} %{buildroot}%{_prefix}/redpesk/%{name}/.rpconfig/manifest.yml


mkdir -p %{buildroot}%{_prefix}/redpesk/%{name}/test/.rpconfig
cp %{SOURCE11} %{buildroot}%{_prefix}/redpesk/%{name}/test/.rpconfig/manifest.yml

mkdir -p %{buildroot}%{_prefix}/redpesk/%{name}/test/etc
mkdir -p %{buildroot}%{_prefix}/redpesk/%{name}/test/bin
cp ./afb-binding/etc/*.yaml %{buildroot}%{_prefix}/redpesk/%{name}/test/etc
cp ./afb-binding/etc/*.sh %{buildroot}%{_prefix}/redpesk/%{name}/test/bin

%files
%dir %{_prefix}/redpesk/%{name}
%dir %{_prefix}/redpesk/%{name}/.rpconfig
%{_prefix}/redpesk/%{name}/.rpconfig/*
%dir %{_prefix}/redpesk/%{name}/lib
%{_prefix}/redpesk/%{name}/lib/*

%changelog