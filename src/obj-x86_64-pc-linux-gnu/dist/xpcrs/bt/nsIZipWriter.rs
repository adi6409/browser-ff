//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/modules/libjar/zipwriter/nsIZipWriter.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIZipWriter",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* attribute ACString comment; */
                    Method {
                        name: "GetComment",
                        params: &[Param { name: "aComment", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetComment",
                        params: &[Param { name: "aComment", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean inQueue; */
                    Method {
                        name: "GetInQueue",
                        params: &[Param { name: "aInQueue", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIFile file; */
                    Method {
                        name: "GetFile",
                        params: &[Param { name: "aFile", ty: "*mut*const nsIFile" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void open (in nsIFile aFile, in int32_t aIoFlags); */
                    Method {
                        name: "Open",
                        params: &[Param { name: "aFile", ty: "*const nsIFile" }, Param { name: "aIoFlags", ty: "int32_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIZipEntry getEntry (in AUTF8String aZipEntry); */
                    Method {
                        name: "GetEntry",
                        params: &[Param { name: "aZipEntry", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut*const nsIZipEntry" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean hasEntry (in AUTF8String aZipEntry); */
                    Method {
                        name: "HasEntry",
                        params: &[Param { name: "aZipEntry", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void addEntryDirectory (in AUTF8String aZipEntry, in PRTime aModTime, in boolean aQueue); */
                    Method {
                        name: "AddEntryDirectory",
                        params: &[Param { name: "aZipEntry", ty: "*const ::nsstring::nsACString" }, Param { name: "aModTime", ty: "PRTime" }, Param { name: "aQueue", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void addEntryFile (in AUTF8String aZipEntry, in int32_t aCompression, in nsIFile aFile, in boolean aQueue); */
                    Method {
                        name: "AddEntryFile",
                        params: &[Param { name: "aZipEntry", ty: "*const ::nsstring::nsACString" }, Param { name: "aCompression", ty: "int32_t" }, Param { name: "aFile", ty: "*const nsIFile" }, Param { name: "aQueue", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void addEntryChannel (in AUTF8String aZipEntry, in PRTime aModTime, in int32_t aCompression, in nsIChannel aChannel, in boolean aQueue); */
                    Method {
                        name: "AddEntryChannel",
                        params: &[Param { name: "aZipEntry", ty: "*const ::nsstring::nsACString" }, Param { name: "aModTime", ty: "PRTime" }, Param { name: "aCompression", ty: "int32_t" }, Param { name: "aChannel", ty: "*const nsIChannel" }, Param { name: "aQueue", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void addEntryStream (in AUTF8String aZipEntry, in PRTime aModTime, in int32_t aCompression, in nsIInputStream aStream, in boolean aQueue); */
                    Method {
                        name: "AddEntryStream",
                        params: &[Param { name: "aZipEntry", ty: "*const ::nsstring::nsACString" }, Param { name: "aModTime", ty: "PRTime" }, Param { name: "aCompression", ty: "int32_t" }, Param { name: "aStream", ty: "*const nsIInputStream" }, Param { name: "aQueue", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void removeEntry (in AUTF8String aZipEntry, in boolean aQueue); */
                    Method {
                        name: "RemoveEntry",
                        params: &[Param { name: "aZipEntry", ty: "*const ::nsstring::nsACString" }, Param { name: "aQueue", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void processQueue (in nsIRequestObserver aObserver, in nsISupports aContext); */
                    Method {
                        name: "ProcessQueue",
                        params: &[Param { name: "aObserver", ty: "*const nsIRequestObserver" }, Param { name: "aContext", ty: "*const nsISupports" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void close (); */
                    Method {
                        name: "Close",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void alignStoredFiles (in uint16_t aAlignSize); */
                    Method {
                        name: "AlignStoredFiles",
                        params: &[Param { name: "aAlignSize", ty: "uint16_t" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

