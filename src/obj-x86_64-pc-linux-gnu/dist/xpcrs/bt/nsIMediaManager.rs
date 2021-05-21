//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/media/nsIMediaManager.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIMediaManagerService",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute nsIArray activeMediaCaptureWindows; */
                    Method {
                        name: "GetActiveMediaCaptureWindows",
                        params: &[Param { name: "aActiveMediaCaptureWindows", ty: "*mut*const nsIArray" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void mediaCaptureWindowState (in nsIDOMWindow aWindow, out unsigned short aCamera, out unsigned short aMicrophone, out unsigned short aScreenShare, out unsigned short aWindowShare, out unsigned short aBrowserShare, out Array<nsIMediaDevice> devices); */
                    Method {
                        name: "MediaCaptureWindowState",
                        params: &[Param { name: "aWindow", ty: "*const nsIDOMWindow" }, Param { name: "aCamera", ty: "*mut u16" }, Param { name: "aMicrophone", ty: "*mut u16" }, Param { name: "aScreenShare", ty: "*mut u16" }, Param { name: "aWindowShare", ty: "*mut u16" }, Param { name: "aBrowserShare", ty: "*mut u16" }, Param { name: "devices", ty: "*mut thin_vec::ThinVec<RefPtr<nsIMediaDevice>>" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void sanitizeDeviceIds (in long long sinceWhen); */
                    Method {
                        name: "SanitizeDeviceIds",
                        params: &[Param { name: "sinceWhen", ty: "i64" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

