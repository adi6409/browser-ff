//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIDownloader.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDownloader",
            base: Some("nsIStreamListener"),
            methods: Ok(&[
                    /* void init (in nsIDownloadObserver observer, in nsIFile downloadLocation); */
                    Method {
                        name: "Init",
                        params: &[Param { name: "observer", ty: "*const nsIDownloadObserver" }, Param { name: "downloadLocation", ty: "*const nsIFile" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIDownloadObserver",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void onDownloadComplete (in nsIDownloader downloader, in nsIRequest request, in nsISupports ctxt, in nsresult status, in nsIFile result); */
                    Method {
                        name: "OnDownloadComplete",
                        params: &[Param { name: "downloader", ty: "*const nsIDownloader" }, Param { name: "request", ty: "*const nsIRequest" }, Param { name: "ctxt", ty: "*const nsISupports" }, Param { name: "status", ty: "::nserror::nsresult" }, Param { name: "result", ty: "*const nsIFile" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

