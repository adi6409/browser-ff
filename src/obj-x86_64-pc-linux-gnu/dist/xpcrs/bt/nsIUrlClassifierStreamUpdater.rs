//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/url-classifier/nsIUrlClassifierStreamUpdater.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIUrlClassifierStreamUpdater",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* boolean downloadUpdates (in ACString aRequestTables, in ACString aRequestPayload, in boolean aIsPostRequest, in ACString aUpdateUrl, in nsIUrlClassifierCallback aSuccessCallback, in nsIUrlClassifierCallback aUpdateErrorCallback, in nsIUrlClassifierCallback aDownloadErrorCallback); */
                    Method {
                        name: "DownloadUpdates",
                        params: &[Param { name: "aRequestTables", ty: "*const ::nsstring::nsACString" }, Param { name: "aRequestPayload", ty: "*const ::nsstring::nsACString" }, Param { name: "aIsPostRequest", ty: "bool" }, Param { name: "aUpdateUrl", ty: "*const ::nsstring::nsACString" }, Param { name: "aSuccessCallback", ty: "*const nsIUrlClassifierCallback" }, Param { name: "aUpdateErrorCallback", ty: "*const nsIUrlClassifierCallback" }, Param { name: "aDownloadErrorCallback", ty: "*const nsIUrlClassifierCallback" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

