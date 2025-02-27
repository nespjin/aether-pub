# Aether Pub Server

https://dart.dev/tools/pub/custom-package-repositories


## Api List
The following are the API endpoints that the [repository-spec-v2](https://github.com/dart-lang/pub/blob/master/doc/repository-spec-v2.md) defines:


 - GET /api/packages/<package>
 - GET /api/packages/versions/new
 - POST /api/packages/upload
 - GET /api/packages/finalize-upload
 - GET /api/packages/<package>/advisories

The following are the API endpoints that the Aether Pub Server supports:

 GET /api/packages/packages-all?<keyword>&<page_size>&<page>&<is_query_all_versions>
 GET /api/packages/<package>/readme
 GET /api/packages/<package>/example
 GET /api/packages/<package>/versions
 GET /api/packages/<package>/changelog
