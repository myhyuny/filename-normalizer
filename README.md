# File name Unicode normalizer

macOS와 Windows 에서 같은 저장공간을 사용하면 Windows에서 한글 자소가 분리되어 골치아픈 경우가 많은데, Microsoft가 Unicode 규격을 구현하다 말아서 벌어진 참사다... (마소 종특)

그래서 Windows 에서 사용하기 위해 유니코드 정규화 작업을 해줘야 해서 대충 만들었다.

## 사용방법

### Terminal

```shell
Usage: normalize [OPTIONS] <PATHS>...

Arguments:
  <PATHS>...  Paths

Options:
  -f <FORM>      Unicode form [default: nfc] [possible values: nfd, nfc, nfkd, nfkc]
  -r             Recursive
  -h, --help     Print help
```
