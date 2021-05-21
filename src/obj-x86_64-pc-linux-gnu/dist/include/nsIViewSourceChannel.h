/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/protocol/viewsource/nsIViewSourceChannel.idl
 */

#ifndef __gen_nsIViewSourceChannel_h__
#define __gen_nsIViewSourceChannel_h__


#ifndef __gen_nsIChannel_h__
#include "nsIChannel.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIViewSourceChannel */
#define NS_IVIEWSOURCECHANNEL_IID_STR "3e9800f8-edb7-4c9a-9285-09b4f045b019"

#define NS_IVIEWSOURCECHANNEL_IID \
  {0x3e9800f8, 0xedb7, 0x4c9a, \
    { 0x92, 0x85, 0x09, 0xb4, 0xf0, 0x45, 0xb0, 0x19 }}

class NS_NO_VTABLE nsIViewSourceChannel : public nsIChannel {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IVIEWSOURCECHANNEL_IID)

  /* [must_use] attribute ACString originalContentType; */
  [[nodiscard]] NS_IMETHOD GetOriginalContentType(nsACString& aOriginalContentType) = 0;
  [[nodiscard]] NS_IMETHOD SetOriginalContentType(const nsACString& aOriginalContentType) = 0;

  /* [must_use] readonly attribute boolean isSrcdocChannel; */
  [[nodiscard]] NS_IMETHOD GetIsSrcdocChannel(bool *aIsSrcdocChannel) = 0;

  /* [must_use] attribute nsIURI baseURI; */
  [[nodiscard]] NS_IMETHOD GetBaseURI(nsIURI **aBaseURI) = 0;
  [[nodiscard]] NS_IMETHOD SetBaseURI(nsIURI *aBaseURI) = 0;

  /* [nostdcall,notxpcom] nsIChannel getInnerChannel (); */
  virtual nsIChannel * GetInnerChannel(void) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIViewSourceChannel, NS_IVIEWSOURCECHANNEL_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIVIEWSOURCECHANNEL \
  [[nodiscard]] NS_IMETHOD GetOriginalContentType(nsACString& aOriginalContentType) override; \
  [[nodiscard]] NS_IMETHOD SetOriginalContentType(const nsACString& aOriginalContentType) override; \
  [[nodiscard]] NS_IMETHOD GetIsSrcdocChannel(bool *aIsSrcdocChannel) override; \
  [[nodiscard]] NS_IMETHOD GetBaseURI(nsIURI **aBaseURI) override; \
  [[nodiscard]] NS_IMETHOD SetBaseURI(nsIURI *aBaseURI) override; \
  virtual nsIChannel * GetInnerChannel(void) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIVIEWSOURCECHANNEL \
  [[nodiscard]] nsresult GetOriginalContentType(nsACString& aOriginalContentType); \
  [[nodiscard]] nsresult SetOriginalContentType(const nsACString& aOriginalContentType); \
  [[nodiscard]] nsresult GetIsSrcdocChannel(bool *aIsSrcdocChannel); \
  [[nodiscard]] nsresult GetBaseURI(nsIURI **aBaseURI); \
  [[nodiscard]] nsresult SetBaseURI(nsIURI *aBaseURI); \
  nsIChannel * GetInnerChannel(void); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIVIEWSOURCECHANNEL(_to) \
  [[nodiscard]] NS_IMETHOD GetOriginalContentType(nsACString& aOriginalContentType) override { return _to GetOriginalContentType(aOriginalContentType); } \
  [[nodiscard]] NS_IMETHOD SetOriginalContentType(const nsACString& aOriginalContentType) override { return _to SetOriginalContentType(aOriginalContentType); } \
  [[nodiscard]] NS_IMETHOD GetIsSrcdocChannel(bool *aIsSrcdocChannel) override { return _to GetIsSrcdocChannel(aIsSrcdocChannel); } \
  [[nodiscard]] NS_IMETHOD GetBaseURI(nsIURI **aBaseURI) override { return _to GetBaseURI(aBaseURI); } \
  [[nodiscard]] NS_IMETHOD SetBaseURI(nsIURI *aBaseURI) override { return _to SetBaseURI(aBaseURI); } \
  virtual nsIChannel * GetInnerChannel(void) override { return _to GetInnerChannel(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIVIEWSOURCECHANNEL(_to) \
  [[nodiscard]] NS_IMETHOD GetOriginalContentType(nsACString& aOriginalContentType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetOriginalContentType(aOriginalContentType); } \
  [[nodiscard]] NS_IMETHOD SetOriginalContentType(const nsACString& aOriginalContentType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetOriginalContentType(aOriginalContentType); } \
  [[nodiscard]] NS_IMETHOD GetIsSrcdocChannel(bool *aIsSrcdocChannel) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsSrcdocChannel(aIsSrcdocChannel); } \
  [[nodiscard]] NS_IMETHOD GetBaseURI(nsIURI **aBaseURI) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetBaseURI(aBaseURI); } \
  [[nodiscard]] NS_IMETHOD SetBaseURI(nsIURI *aBaseURI) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetBaseURI(aBaseURI); } \
  virtual nsIChannel * GetInnerChannel(void) override; 


#endif /* __gen_nsIViewSourceChannel_h__ */
