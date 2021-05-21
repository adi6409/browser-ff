/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/protocol/ftp/nsIFTPChannel.idl
 */

#ifndef __gen_nsIFTPChannel_h__
#define __gen_nsIFTPChannel_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIFTPChannel */
#define NS_IFTPCHANNEL_IID_STR "07f0d5cd-1fd5-4aa3-b6fc-665bdc5dbf9f"

#define NS_IFTPCHANNEL_IID \
  {0x07f0d5cd, 0x1fd5, 0x4aa3, \
    { 0xb6, 0xfc, 0x66, 0x5b, 0xdc, 0x5d, 0xbf, 0x9f }}

class NS_NO_VTABLE nsIFTPChannel : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IFTPCHANNEL_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIFTPChannel;

  /* attribute PRTime lastModifiedTime; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetLastModifiedTime(PRTime *aLastModifiedTime) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetLastModifiedTime(PRTime aLastModifiedTime) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIFTPChannel, NS_IFTPCHANNEL_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIFTPCHANNEL \
  NS_IMETHOD GetLastModifiedTime(PRTime *aLastModifiedTime) override; \
  NS_IMETHOD SetLastModifiedTime(PRTime aLastModifiedTime) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIFTPCHANNEL \
  nsresult GetLastModifiedTime(PRTime *aLastModifiedTime); \
  nsresult SetLastModifiedTime(PRTime aLastModifiedTime); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIFTPCHANNEL(_to) \
  NS_IMETHOD GetLastModifiedTime(PRTime *aLastModifiedTime) override { return _to GetLastModifiedTime(aLastModifiedTime); } \
  NS_IMETHOD SetLastModifiedTime(PRTime aLastModifiedTime) override { return _to SetLastModifiedTime(aLastModifiedTime); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIFTPCHANNEL(_to) \
  NS_IMETHOD GetLastModifiedTime(PRTime *aLastModifiedTime) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLastModifiedTime(aLastModifiedTime); } \
  NS_IMETHOD SetLastModifiedTime(PRTime aLastModifiedTime) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetLastModifiedTime(aLastModifiedTime); } 


/* starting interface:    nsIFTPEventSink */
#define NS_IFTPEVENTSINK_IID_STR "455d4234-0330-43d2-bbfb-99afbecbfeb0"

#define NS_IFTPEVENTSINK_IID \
  {0x455d4234, 0x0330, 0x43d2, \
    { 0xbb, 0xfb, 0x99, 0xaf, 0xbe, 0xcb, 0xfe, 0xb0 }}

class NS_NO_VTABLE nsIFTPEventSink : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IFTPEVENTSINK_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIFTPEventSink;

  /* void OnFTPControlLog (in boolean server, in string msg); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnFTPControlLog(bool server, const char * msg) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIFTPEventSink, NS_IFTPEVENTSINK_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIFTPEVENTSINK \
  NS_IMETHOD OnFTPControlLog(bool server, const char * msg) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIFTPEVENTSINK \
  nsresult OnFTPControlLog(bool server, const char * msg); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIFTPEVENTSINK(_to) \
  NS_IMETHOD OnFTPControlLog(bool server, const char * msg) override { return _to OnFTPControlLog(server, msg); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIFTPEVENTSINK(_to) \
  NS_IMETHOD OnFTPControlLog(bool server, const char * msg) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnFTPControlLog(server, msg); } 


#endif /* __gen_nsIFTPChannel_h__ */
