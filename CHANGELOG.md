## 1.1.0 - 2020 Nov 14
- Added units of frequency
- Added support using foot-inch syntax with addition, like `2"+6'4"`
- Unsupported foot-inch syntax like `(6)'4"` and `6'4!"` now cause errors
- Fixed README.md stating the performance is 1000x slower than it actually is
- Fixed trailing percentage signs being ignored when `allow_trailing_operators` is true
- Fixed error caused by consecutive percentage signs

## 1.0.2 - 2020 Oct 12
- Fix parsing of unit `Quarter` (#1)
- Use division instead of multiplication when dividing numbers of the same unit `Quarter` (#1)

## 1.0.1 - 2020 Aug 20
- Fixed the library not working
- Added documentation comments
- Added docs.rs documentation link
- Various fixes and improvements

## 1.0.0 - 2020 Aug 20
- Initial release
