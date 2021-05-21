/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIInputStreamChannel.idl
 */

#ifndef __gen_nsIInputStreamChannel_h__
#define __gen_nsIInputStreamChannel_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIInputStream; /* forward declaration */

class nsIURI; /* forward declaration */


/* starting interface:    nsIInputStreamChannel */
#define NS_IINPUTSTREAMCHANNEL_IID_STR "ea730238-4bfd-4015-8489-8f264d05b343"

#define NS_IINPUTSTREAMCHANNEL_IID \
  {0xea730238, 0x4bfd, 0x4015, \
    { 0x84, 0x89, 0x8f, 0x26, 0x4d, 0x05, 0xb3, 0x43 }}

class NS_NO_VTABLE nsIInputStreamChannel : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IINPUTSTREAMCHANNEL_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIInputStreamChannel;

  /* void setURI (in nsIURI aURI); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetURI(nsIURI *aURI) = 0;

  /* attribute nsIInputStream contentStream; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetContentStream(nsIInputStream **aContentStream) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetContentStream(nsIInputStream *aContentStream) = 0;

  /* attribute AString srcdocData; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSrcdocData(nsAString& aSrcdocData) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetSrcdocData(const nsAString& aSrcdocData) = 0;

  /* readonly attribute boolean isSrcdocChannel; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetIsSrcdocChannel(bool *aIsSrcdocChannel) = 0;

  /* attribute nsIURI baseURI; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetBaseURI(nsIURI **aBaseURI) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetBaseURI(nsIURI *aBaseURI) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIInputStreamChannel, NS_IINPUTSTREAMCHANNEL_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIINPUTSTREAMCHANNEL \
  NS_IMETHOD SetURI(nsIURI *aURI) override; \
  NS_IMETHOD GetContentStream(nsIInputStream **aContentStream) override; \
  NS_IMETHOD SetContentStream(nsIInputStream *aContentStream) override; \
  NS_IMETHOD GetSrcdocData(nsAString& aSrcdocData) override; \
  NS_IMETHOD SetSrcdocData(const nsAString& aSrcdocData) override; \
  NS_IMETHOD GetIsSrcdocChannel(bool *aIsSrcdocChannel) override; \
  NS_IMETHOD GetBaseURI(nsIURI **aBaseURI) override; \
  NS_IMETHOD SetBaseURI(nsIURI *aBaseURI) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIINPUTSTREAMCHANNEL \
  nsresult SetURI(nsIURI *aURI); \
  nsresult GetContentStream(nsIInputStream **aContentStream); \
  nsresult SetContentStream(nsIInputStream *aContentStream); \
  nsresult GetSrcdocData(nsAString& aSrcdocData); \
  nsresult SetSrcdocData(const nsAString& aSrcdocData); \
  nsresult GetIsSrcdocChannel(bool *aIsSrcdocChannel); \
  nsresult GetBaseURI(nsIURI **aBaseURI); \
  nsresult SetBaseURI(nsIURI *aBaseURI); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIINPUTSTREAMCHANNEL(_to) \
  NS_IMETHOD SetURI(nsIURI *aURI) override { return _to SetURI(aURI); } \
  NS_IMETHOD GetContentStream(nsIInputStream **aContentStream) override { return _to GetContentStream(aContentStream); } \
  NS_IMETHOD SetContentStream(nsIInputStream *aContentStream) override { return _to SetContentStream(aContentStream); } \
  NS_IMETHOD GetSrcdocData(nsAString& aSrcdocData) override { return _to GetSrcdocData(aSrcdocData); } \
  NS_IMETHOD SetSrcdocData(const nsAString& aSrcdocData) override { return _to SetSrcdocData(aSrcdocData); } \
  NS_IMETHOD GetIsSrcdocChannel(bool *aIsSrcdocChannel) override { return _to GetIsSrcdocChannel(aIsSrcdocChannel); } \
  NS_IMETHOD GetBaseURI(nsIURI **aBaseURI) override { return _to GetBaseURI(aBaseURI); } \
  NS_IMETHOD SetBaseURI(nsIURI *aBaseURI) override { return _to SetBaseURI(aBaseURI); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIINPUTSTREAMCHANNEL(_to) \
  NS_IMETHOD SetURI(nsIURI *aURI) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetURI(aURI); } \
  NS_IMETHOD GetContentStream(nsIInputStream **aContentStream) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetContentStream(aContentStream); } \
  NS_IMETHOD SetContentStream(nsIInputStream *aContentStream) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetContentStream(aContentStream); } \
  NS_IMETHOD GetSrcdocData(nsAString& aSrcdocData) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSrcdocData(aSrcdocData); } \
  NS_IMETHOD SetSrcdocData(const nsAString& aSrcdocData) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetSrcdocData(aSrcdocData); } \
  NS_IMETHOD GetIsSrcdocChannel(bool *aIsSrcdocChannel) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsSrcdocChannel(aIsSrcdocChannel); } \
  NS_IMETHOD GetBaseURI(nsIURI **aBaseURI) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetBaseURI(aBaseURI); } \
  NS_IMETHOD SetBaseURI(nsIURI *aBaseURI) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetBaseURI(aBaseURI); } 


#endif /* __gen_nsIInputStreamChannel_h__ */
