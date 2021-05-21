//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/modules/nsIBrowserWindowTracker.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIVisibleTab",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* attribute AString contentTitle; */
                    Method {
                        name: "GetContentTitle",
                        params: &[Param { name: "aContentTitle", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetContentTitle",
                        params: &[Param { name: "aContentTitle", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute int64_t browserId; */
                    Method {
                        name: "GetBrowserId",
                        params: &[Param { name: "aBrowserId", ty: "*mut int64_t" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetBrowserId",
                        params: &[Param { name: "aBrowserId", ty: "int64_t" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIBrowserWindowTracker",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* Array<nsIVisibleTab> getAllVisibleTabs (); */
                    Method {
                        name: "GetAllVisibleTabs",
                        params: &[Param { name: "_retval", ty: "*mut thin_vec::ThinVec<RefPtr<nsIVisibleTab>>" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

