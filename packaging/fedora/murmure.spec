# Spec file based on the work of Ines WALLON <missd@drupalista.dev>
# Original: https://gitlab.famillewallon.com/rpm-packages/murmure
# Adapted for in-tree CI builds (Source1/2 removed; Source0 kept as placeholder
# only to satisfy %%setup; sources are placed by the workflow in
# ~/rpmbuild/BUILD/%%{name}-%%{version} prior to rpmbuild).

%define         model_name  parakeet-tdt-0.6b-v3-int8

Name:           murmure
Version:        %{version}
Release:        1%{?dist}
Summary:        Privacy-first speech-to-text, running entirely on your machine
Group:          Applications/Productivity
License:        AGPL-3.0-or-later
URL:            https://github.com/Kieirra/murmure
# Placeholder source declaration: required by rpmbuild because %setup -T -D
# references Source0 implicitly. The file is not used (sources are pre-staged
# by the CI workflow into ~/rpmbuild/BUILD/ — see %prep below).
Source0:        %{name}-%{version}.tar.gz
BuildRequires:  openssl-devel
BuildRequires:  libappindicator-gtk3-devel
BuildRequires:  rust-alsa-devel
BuildRequires:  libappindicator-devel
BuildRequires:  libsoup3-devel
BuildRequires:  javascriptcoregtk4.1-devel
BuildRequires:  webkit2gtk4.1-devel
BuildRequires:  libstdc++-static

Requires:       %{name}-data = %{version}
Requires:       gdk-pixbuf2
Requires:       desktop-file-utils
Requires:       glib2
Requires:       gtk3
Requires:       hicolor-icon-theme
Requires:       libsoup
Requires:       pango
Requires:       webkit2gtk4.1

%description
A privacy-first, open-source speech-to-text application that runs entirely on
your machine, powered by a neural network via NVIDIA's Parakeet TDT 0.6B v3 model
for fast, local transcription. Murmure turns your voice into text with no internet
connection and zero data collection, and supports 25 European languages.

%package data
Summary:        Data files for %{name}
BuildArch:      noarch
Group:          Applications/Productivity
Requires:       %{name}
License:        CC-BY-4.0

%description data
This package contains data files essential to run Murmure

%prep
# Sources are pre-staged by the CI workflow into the build directory:
#   ~/rpmbuild/BUILD/%%{name}-%%{version}/                     <- repo checkout
#   ~/rpmbuild/BUILD/%%{name}-%%{version}/%%{model_name}/      <- ONNX model
#   ~/rpmbuild/BUILD/%%{name}-%%{version}/murmure.desktop      <- desktop entry
# %%setup is intentionally skipped (no tarball to unpack).
%setup -T -D -n %{name}-%{version}
pnpm install

%build
CFLAGS+=' -ffat-lto-objects'
pnpm tauri build --no-bundle

%install
install -dm755 %{buildroot}%{_bindir}
install -m755 src-tauri/target/release/%{name} %{buildroot}%{_bindir}/%{name}

mkdir -p %{buildroot}%{_prefix}/lib/%{name}/_up_/resources/

cp -r src-tauri/target/release/_up_/ %{buildroot}%{_prefix}/lib/%{name}
cp -r %{model_name}/ %{buildroot}%{_prefix}/lib/%{name}/_up_/resources/

install -d %{buildroot}%{_datadir}/icons/hicolor/128x128/apps
install -m644 src-tauri/icons/128x128.png  %{buildroot}%{_datadir}/icons/hicolor/128x128/apps/%{name}.png

install -d %{buildroot}%{_datadir}/icons/hicolor/256x256/apps
install -m644 src-tauri/icons/256x256.png  %{buildroot}%{_datadir}/icons/hicolor/256x256/apps/%{name}.png

install -d %{buildroot}%{_datadir}/icons/hicolor/512x512/apps
install -m644 src-tauri/icons/icon.png  %{buildroot}%{_datadir}/icons/hicolor/512x512/apps/%{name}.png

install -d %{buildroot}%{_datadir}/applications
install -m644 %{name}.desktop %{buildroot}%{_datadir}/applications/%{name}.desktop

%files
%defattr(-,root,root,-)
%license LICENSE
%doc README.md
%{_bindir}/%{name}
%{_datadir}/applications/%{name}.desktop
%{_datadir}/icons/hicolor/*
/%{_prefix}/lib/%{name}/_up_/resources/audio/*
/%{_prefix}/lib/%{name}/_up_/resources/cc-rules/*

%files data
%defattr(-,root,root,-)
/%{_prefix}/lib/%{name}/_up_/resources/%{model_name}

%changelog
* Sat May 09 2026 Murmure CI <release@murmure> - 1.8.99-1
- Adapt spec for in-tree CI builds (no external Source URLs).
- Bundle local desktop file from packaging/fedora/.
* Mon Mar 30 2026 Ines WALLON <missd@drupalista.dev> - 1.8.0-1
- Update to 1.8.0.
* Sun Mar 01 2026 Ines WALLON <missd@drupalista.dev> - 1.7.0-1
- Init
