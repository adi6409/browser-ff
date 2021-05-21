//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/modules/libjar/nsIZipReader.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIZipEntry",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute unsigned short compression; */
                    Method {
                        name: "GetCompression",
                        params: &[Param { name: "aCompression", ty: "*mut u16" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute unsigned long size; */
                    Method {
                        name: "GetSize",
                        params: &[Param { name: "aSize", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute unsigned long realSize; */
                    Method {
                        name: "GetRealSize",
                        params: &[Param { name: "aRealSize", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute unsigned long CRC32; */
                    Method {
                        name: "GetCRC32",
                        params: &[Param { name: "aCRC32", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean isDirectory; */
                    Method {
                        name: "GetIsDirectory",
                        params: &[Param { name: "aIsDirectory", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute PRTime lastModifiedTime; */
                    Method {
                        name: "GetLastModifiedTime",
                        params: &[Param { name: "aLastModifiedTime", ty: "*mut PRTime" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean isSynthetic; */
                    Method {
                        name: "GetIsSynthetic",
                        params: &[Param { name: "aIsSynthetic", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute unsigned long permissions; */
                    Method {
                        name: "GetPermissions",
                        params: &[Param { name: "aPermissions", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIZipReader",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void open (in nsIFile zipFile); */
                    Method {
                        name: "Open",
                        params: &[Param { name: "zipFile", ty: "*const nsIFile" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void openInner (in nsIZipReader zipReader, in AUTF8String zipEntry); */
                    Method {
                        name: "OpenInner",
                        params: &[Param { name: "zipReader", ty: "*const nsIZipReader" }, Param { name: "zipEntry", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void openMemory (in voidPtr aData, in unsigned long aLength); */
                    Method {
                        name: "OpenMemory",
                        params: &[Param { name: "aData", ty: "*const libc::c_void" }, Param { name: "aLength", ty: "u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIFile file; */
                    Method {
                        name: "GetFile",
                        params: &[Param { name: "aFile", ty: "*mut*const nsIFile" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void close (); */
                    Method {
                        name: "Close",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void test (in AUTF8String aEntryName); */
                    Method {
                        name: "Test",
                        params: &[Param { name: "aEntryName", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void extract (in AUTF8String zipEntry, in nsIFile outFile); */
                    Method {
                        name: "Extract",
                        params: &[Param { name: "zipEntry", ty: "*const ::nsstring::nsACString" }, Param { name: "outFile", ty: "*const nsIFile" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIZipEntry getEntry (in AUTF8String zipEntry); */
                    Method {
                        name: "GetEntry",
                        params: &[Param { name: "zipEntry", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut *const nsIZipEntry" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean hasEntry (in AUTF8String zipEntry); */
                    Method {
                        name: "HasEntry",
                        params: &[Param { name: "zipEntry", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIUTF8StringEnumerator findEntries (in AUTF8String aPattern); */
                    Method {
                        name: "FindEntries",
                        params: &[Param { name: "aPattern", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut*const nsIUTF8StringEnumerator" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIInputStream getInputStream (in AUTF8String zipEntry); */
                    Method {
                        name: "GetInputStream",
                        params: &[Param { name: "zipEntry", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut*const nsIInputStream" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIInputStream getInputStreamWithSpec (in AUTF8String aJarSpec, in AUTF8String zipEntry); */
                    Method {
                        name: "GetInputStreamWithSpec",
                        params: &[Param { name: "aJarSpec", ty: "*const ::nsstring::nsACString" }, Param { name: "zipEntry", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut*const nsIInputStream" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIZipReaderCache",
            base: Some("nsISupports"),
            methods: Err("native type PRFileDesc unsupported"),
        },

        ]; D}

