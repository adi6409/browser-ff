/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIStreamTransportService.idl
 */

#ifndef __gen_nsIStreamTransportService_h__
#define __gen_nsIStreamTransportService_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsITransport; /* forward declaration */

class nsIInputStream; /* forward declaration */

class nsIOutputStream; /* forward declaration */

class nsIInputAvailableCallback; /* forward declaration */


/* starting interface:    nsIStreamTransportService */
#define NS_ISTREAMTRANSPORTSERVICE_IID_STR "5e0adf7d-9785-45c3-a193-04f25a75da8f"

#define NS_ISTREAMTRANSPORTSERVICE_IID \
  {0x5e0adf7d, 0x9785, 0x45c3, \
    { 0xa1, 0x93, 0x04, 0xf2, 0x5a, 0x75, 0xda, 0x8f }}

class NS_NO_VTABLE nsIStreamTransportService : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISTREAMTRANSPORTSERVICE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIStreamTransportService;

  /* nsITransport createInputTransport (in nsIInputStream aStream, in boolean aCloseWhenDone); */
  NS_IMETHOD CreateInputTransport(nsIInputStream *aStream, bool aCloseWhenDone, nsITransport **_retval) = 0;

  /* void InputAvailable (in nsIInputStream aStream, in nsIInputAvailableCallback aCallback); */
  NS_IMETHOD InputAvailable(nsIInputStream *aStream, nsIInputAvailableCallback *aCallback) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIStreamTransportService, NS_ISTREAMTRANSPORTSERVICE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISTREAMTRANSPORTSERVICE \
  NS_IMETHOD CreateInputTransport(nsIInputStream *aStream, bool aCloseWhenDone, nsITransport **_retval) override; \
  NS_IMETHOD InputAvailable(nsIInputStream *aStream, nsIInputAvailableCallback *aCallback) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISTREAMTRANSPORTSERVICE \
  nsresult CreateInputTransport(nsIInputStream *aStream, bool aCloseWhenDone, nsITransport **_retval); \
  nsresult InputAvailable(nsIInputStream *aStream, nsIInputAvailableCallback *aCallback); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISTREAMTRANSPORTSERVICE(_to) \
  NS_IMETHOD CreateInputTransport(nsIInputStream *aStream, bool aCloseWhenDone, nsITransport **_retval) override { return _to CreateInputTransport(aStream, aCloseWhenDone, _retval); } \
  NS_IMETHOD InputAvailable(nsIInputStream *aStream, nsIInputAvailableCallback *aCallback) override { return _to InputAvailable(aStream, aCallback); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISTREAMTRANSPORTSERVICE(_to) \
  NS_IMETHOD CreateInputTransport(nsIInputStream *aStream, bool aCloseWhenDone, nsITransport **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CreateInputTransport(aStream, aCloseWhenDone, _retval); } \
  NS_IMETHOD InputAvailable(nsIInputStream *aStream, nsIInputAvailableCallback *aCallback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->InputAvailable(aStream, aCallback); } 


/* starting interface:    nsIInputAvailableCallback */
#define NS_IINPUTAVAILABLECALLBACK_IID_STR "ff2da731-44d0-4dd9-8236-c99387fec721"

#define NS_IINPUTAVAILABLECALLBACK_IID \
  {0xff2da731, 0x44d0, 0x4dd9, \
    { 0x82, 0x36, 0xc9, 0x93, 0x87, 0xfe, 0xc7, 0x21 }}

class NS_NO_VTABLE nsIInputAvailableCallback : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IINPUTAVAILABLECALLBACK_IID)

  /* void onInputAvailableComplete (in unsigned long long available, in nsresult available_return_code); */
  NS_IMETHOD OnInputAvailableComplete(uint64_t available, nsresult available_return_code) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIInputAvailableCallback, NS_IINPUTAVAILABLECALLBACK_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIINPUTAVAILABLECALLBACK \
  NS_IMETHOD OnInputAvailableComplete(uint64_t available, nsresult available_return_code) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIINPUTAVAILABLECALLBACK \
  nsresult OnInputAvailableComplete(uint64_t available, nsresult available_return_code); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIINPUTAVAILABLECALLBACK(_to) \
  NS_IMETHOD OnInputAvailableComplete(uint64_t available, nsresult available_return_code) override { return _to OnInputAvailableComplete(available, available_return_code); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIINPUTAVAILABLECALLBACK(_to) \
  NS_IMETHOD OnInputAvailableComplete(uint64_t available, nsresult available_return_code) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnInputAvailableComplete(available, available_return_code); } 


#endif /* __gen_nsIStreamTransportService_h__ */
