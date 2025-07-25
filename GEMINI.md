# 프로젝트: rhytry

## 1. 개요

`rhytry`는 Rust 언어와 Bevy 게임 엔진을 사용하여 개발하는 리듬 게임입니다. Bevy의 ECS(Entity-Component-System) 아키텍처와 플러그인(Plugin), 상태(State) 기반으로 코드가 구성됩니다.

## 2. 주요 기술 및 도구

*   **언어**: Rust
*   **게임 엔진**: Bevy

## 3. 주요 게임 개념

*   **`Onpu`**: 게임 내 음표를 나타내는 컴포넌트.

## 4. 프로젝트 구조

```
assets/              # 게임 애셋(이미지, 사운드 등)을 저장합니다.
  ⨽ audio/           # 배경 음악, 효과음 등 오디오 파일을 저장합니다.
  ⨽ texture/         # 게임에 사용되는 텍스처 파일을 저장합니다.
build/               # 빌드 관련 스크립트 및 설정 파일을 저장합니다.
  ⨽ web/             # 웹 빌드 관련 파일을 저장합니다.
  ⨽ windows/         # Windows 빌드 관련 파일을 저장합니다.
credits/             # 라이선스 및 크레딧 정보를 저장합니다.
dist/                # 웹 빌드 결과물이 저장되는 디렉터리입니다.
src/                 # Rust 소스 코드를 저장합니다.
  ⨽ infrastructure/  # 인프라 관련 코드를 모아놓은 모듈입니다.
  ⨽ lib.rs           # 라이브러리의 최상위 파일입니다.
  ⨽ main.rs          # 실행 파일의 최상위 파일입니다.
.editorconfig        # 다양한 편집기 및 IDE에서 코드 스타일을 일관되게 유지하기 위한 설정 파일입니다.
.gitattributes       # Git이 파일 경로에 따라 특정 속성을 적용하도록 설정하는 파일입니다.
.gitignore           # Git이 추적하지 않을 파일 및 디렉터리를 지정하는 파일입니다.
Cargo.lock           # 프로젝트의 의존성 버전을 고정하는 파일입니다.
Cargo.toml           # Rust 프로젝트의 메타데이터 및 의존성을 관리하는 파일입니다.
index.html           # 웹 빌드의 진입점 역할을 하는 HTML 파일입니다.
LICENSE              # 프로젝트 라이선스 파일입니다.
mise.toml            # `mise`를 사용하여 프로젝트의 개발 환경을 관리하는 설정 파일입니다.
rustfmt.toml         # `rustfmt` 도구를 사용하여 코드 스타일을 일관되게 유지하기 위한 설정 파일입니다.
Trunk.toml           # `Trunk`를 사용하여 웹 빌드를 관리하는 설정 파일입니다.
```
