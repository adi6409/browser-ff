/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/network/interfaces/nsIUDPSocketChild.idl
 */

#ifndef __gen_nsIUDPSocketChild_h__
#define __gen_nsIUDPSocketChild_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsINetAddr_h__
#include "nsINetAddr.h"
#endif

#ifndef __gen_nsTArray_h__
#include "nsTArray.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIUDPSocketInternal; /* forward declaration */

class nsIInputStream; /* forward declaration */

class nsIPrincipal; /* forward declaration */

class nsIEventTarget; /* forward declaration */


/* starting interface:    nsIUDPSocketInternal */
#define NS_IUDPSOCKETINTERNAL_IID_STR "613dd3ad-598b-4da9-ad63-bbda50c20098"

#define NS_IUDPSOCKETINTERNAL_IID \
  {0x613dd3ad, 0x598b, 0x4da9, \
    { 0xad, 0x63, 0xbb, 0xda, 0x50, 0xc2, 0x00, 0x98 }}

class NS_NO_VTABLE nsIUDPSocketInternal : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IUDPSOCKETINTERNAL_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIUDPSocketInternal;

  /* void callListenerOpened (); */
  NS_IMETHOD CallListenerOpened(void) = 0;

  /* void callListenerConnected (); */
  NS_IMETHOD CallListenerConnected(void) = 0;

  /* void callListenerClosed (); */
  NS_IMETHOD CallListenerClosed(void) = 0;

  /* void callListenerReceivedData (in AUTF8String host, in unsigned short port, in Array<uint8_t> data); */
  NS_IMETHOD CallListenerReceivedData(const nsACString& host, uint16_t port, const nsTArray<uint8_t >& data) = 0;

  /* void callListenerError (in AUTF8String message, in AUTF8String filename, in uint32_t lineNumber); */
  NS_IMETHOD CallListenerError(const nsACString& message, const nsACString& filename, uint32_t lineNumber) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIUDPSocketInternal, NS_IUDPSOCKETINTERNAL_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIUDPSOCKETINTERNAL \
  NS_IMETHOD CallListenerOpened(void) override; \
  NS_IMETHOD CallListenerConnected(void) override; \
  NS_IMETHOD CallListenerClosed(void) override; \
  NS_IMETHOD CallListenerReceivedData(const nsACString& host, uint16_t port, const nsTArray<uint8_t >& data) override; \
  NS_IMETHOD CallListenerError(const nsACString& message, const nsACString& filename, uint32_t lineNumber) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIUDPSOCKETINTERNAL \
  nsresult CallListenerOpened(void); \
  nsresult CallListenerConnected(void); \
  nsresult CallListenerClosed(void); \
  nsresult CallListenerReceivedData(const nsACString& host, uint16_t port, const nsTArray<uint8_t >& data); \
  nsresult CallListenerError(const nsACString& message, const nsACString& filename, uint32_t lineNumber); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIUDPSOCKETINTERNAL(_to) \
  NS_IMETHOD CallListenerOpened(void) override { return _to CallListenerOpened(); } \
  NS_IMETHOD CallListenerConnected(void) override { return _to CallListenerConnected(); } \
  NS_IMETHOD CallListenerClosed(void) override { return _to CallListenerClosed(); } \
  NS_IMETHOD CallListenerReceivedData(const nsACString& host, uint16_t port, const nsTArray<uint8_t >& data) override { return _to CallListenerReceivedData(host, port, data); } \
  NS_IMETHOD CallListenerError(const nsACString& message, const nsACString& filename, uint32_t lineNumber) override { return _to CallListenerError(message, filename, lineNumber); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIUDPSOCKETINTERNAL(_to) \
  NS_IMETHOD CallListenerOpened(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CallListenerOpened(); } \
  NS_IMETHOD CallListenerConnected(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CallListenerConnected(); } \
  NS_IMETHOD CallListenerClosed(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CallListenerClosed(); } \
  NS_IMETHOD CallListenerReceivedData(const nsACString& host, uint16_t port, const nsTArray<uint8_t >& data) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CallListenerReceivedData(host, port, data); } \
  NS_IMETHOD CallListenerError(const nsACString& message, const nsACString& filename, uint32_t lineNumber) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CallListenerError(message, filename, lineNumber); } 


#endif /* __gen_nsIUDPSocketChild_h__ */
