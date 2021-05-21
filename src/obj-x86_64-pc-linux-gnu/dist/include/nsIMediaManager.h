/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/media/nsIMediaManager.idl
 */

#ifndef __gen_nsIMediaManager_h__
#define __gen_nsIMediaManager_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsTArray_h__
#include "nsTArray.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIArray; /* forward declaration */

class nsIDOMWindow; /* forward declaration */

class nsIMediaDevice; /* forward declaration */

#define NS_MEDIAMANAGERSERVICE_CID {0xabc622ea, 0x9655, 0x4123, {0x80, 0xd9, 0x22, 0x62, 0x1b, 0xdd, 0x54, 0x65}}
#define MEDIAMANAGERSERVICE_CONTRACTID "@mozilla.org/mediaManagerService;1"

/* starting interface:    nsIMediaManagerService */
#define NS_IMEDIAMANAGERSERVICE_IID_STR "24b23e01-33fd-401f-ba25-6e52658750b0"

#define NS_IMEDIAMANAGERSERVICE_IID \
  {0x24b23e01, 0x33fd, 0x401f, \
    { 0xba, 0x25, 0x6e, 0x52, 0x65, 0x87, 0x50, 0xb0 }}

class NS_NO_VTABLE nsIMediaManagerService : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IMEDIAMANAGERSERVICE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIMediaManagerService;

  /* readonly attribute nsIArray activeMediaCaptureWindows; */
  NS_IMETHOD GetActiveMediaCaptureWindows(nsIArray **aActiveMediaCaptureWindows) = 0;

  enum {
    STATE_NOCAPTURE = 0U,
    STATE_CAPTURE_ENABLED = 1U,
    STATE_CAPTURE_DISABLED = 2U
  };

  /* void mediaCaptureWindowState (in nsIDOMWindow aWindow, out unsigned short aCamera, out unsigned short aMicrophone, out unsigned short aScreenShare, out unsigned short aWindowShare, out unsigned short aBrowserShare, out Array<nsIMediaDevice> devices); */
  NS_IMETHOD MediaCaptureWindowState(nsIDOMWindow *aWindow, uint16_t *aCamera, uint16_t *aMicrophone, uint16_t *aScreenShare, uint16_t *aWindowShare, uint16_t *aBrowserShare, nsTArray<RefPtr<nsIMediaDevice>>& devices) = 0;

  /* void sanitizeDeviceIds (in long long sinceWhen); */
  NS_IMETHOD SanitizeDeviceIds(int64_t sinceWhen) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIMediaManagerService, NS_IMEDIAMANAGERSERVICE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIMEDIAMANAGERSERVICE \
  NS_IMETHOD GetActiveMediaCaptureWindows(nsIArray **aActiveMediaCaptureWindows) override; \
  NS_IMETHOD MediaCaptureWindowState(nsIDOMWindow *aWindow, uint16_t *aCamera, uint16_t *aMicrophone, uint16_t *aScreenShare, uint16_t *aWindowShare, uint16_t *aBrowserShare, nsTArray<RefPtr<nsIMediaDevice>>& devices) override; \
  NS_IMETHOD SanitizeDeviceIds(int64_t sinceWhen) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIMEDIAMANAGERSERVICE \
  nsresult GetActiveMediaCaptureWindows(nsIArray **aActiveMediaCaptureWindows); \
  nsresult MediaCaptureWindowState(nsIDOMWindow *aWindow, uint16_t *aCamera, uint16_t *aMicrophone, uint16_t *aScreenShare, uint16_t *aWindowShare, uint16_t *aBrowserShare, nsTArray<RefPtr<nsIMediaDevice>>& devices); \
  nsresult SanitizeDeviceIds(int64_t sinceWhen); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIMEDIAMANAGERSERVICE(_to) \
  NS_IMETHOD GetActiveMediaCaptureWindows(nsIArray **aActiveMediaCaptureWindows) override { return _to GetActiveMediaCaptureWindows(aActiveMediaCaptureWindows); } \
  NS_IMETHOD MediaCaptureWindowState(nsIDOMWindow *aWindow, uint16_t *aCamera, uint16_t *aMicrophone, uint16_t *aScreenShare, uint16_t *aWindowShare, uint16_t *aBrowserShare, nsTArray<RefPtr<nsIMediaDevice>>& devices) override { return _to MediaCaptureWindowState(aWindow, aCamera, aMicrophone, aScreenShare, aWindowShare, aBrowserShare, devices); } \
  NS_IMETHOD SanitizeDeviceIds(int64_t sinceWhen) override { return _to SanitizeDeviceIds(sinceWhen); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIMEDIAMANAGERSERVICE(_to) \
  NS_IMETHOD GetActiveMediaCaptureWindows(nsIArray **aActiveMediaCaptureWindows) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetActiveMediaCaptureWindows(aActiveMediaCaptureWindows); } \
  NS_IMETHOD MediaCaptureWindowState(nsIDOMWindow *aWindow, uint16_t *aCamera, uint16_t *aMicrophone, uint16_t *aScreenShare, uint16_t *aWindowShare, uint16_t *aBrowserShare, nsTArray<RefPtr<nsIMediaDevice>>& devices) override { return !_to ? NS_ERROR_NULL_POINTER : _to->MediaCaptureWindowState(aWindow, aCamera, aMicrophone, aScreenShare, aWindowShare, aBrowserShare, devices); } \
  NS_IMETHOD SanitizeDeviceIds(int64_t sinceWhen) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SanitizeDeviceIds(sinceWhen); } 


#endif /* __gen_nsIMediaManager_h__ */
