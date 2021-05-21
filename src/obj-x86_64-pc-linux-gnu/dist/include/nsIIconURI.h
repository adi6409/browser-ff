/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/image/nsIIconURI.idl
 */

#ifndef __gen_nsIIconURI_h__
#define __gen_nsIIconURI_h__


#ifndef __gen_nsIURL_h__
#include "nsIURL.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIMozIconURI */
#define NS_IMOZICONURI_IID_STR "f8fe5ef2-5f2b-43f3-857d-5b64d192c427"

#define NS_IMOZICONURI_IID \
  {0xf8fe5ef2, 0x5f2b, 0x43f3, \
    { 0x85, 0x7d, 0x5b, 0x64, 0xd1, 0x92, 0xc4, 0x27 }}

class NS_NO_VTABLE nsIMozIconURI : public nsIURI {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IMOZICONURI_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIMozIconURI;

  /* readonly attribute nsIURL iconURL; */
  NS_IMETHOD GetIconURL(nsIURL **aIconURL) = 0;

  /* readonly attribute unsigned long imageSize; */
  NS_IMETHOD GetImageSize(uint32_t *aImageSize) = 0;

  /* readonly attribute ACString stockIcon; */
  NS_IMETHOD GetStockIcon(nsACString& aStockIcon) = 0;

  /* readonly attribute ACString iconSize; */
  NS_IMETHOD GetIconSize(nsACString& aIconSize) = 0;

  /* readonly attribute ACString iconState; */
  NS_IMETHOD GetIconState(nsACString& aIconState) = 0;

  /* readonly attribute ACString contentType; */
  NS_IMETHOD GetContentType(nsACString& aContentType) = 0;

  /* readonly attribute ACString fileExtension; */
  NS_IMETHOD GetFileExtension(nsACString& aFileExtension) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIMozIconURI, NS_IMOZICONURI_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIMOZICONURI \
  NS_IMETHOD GetIconURL(nsIURL **aIconURL) override; \
  NS_IMETHOD GetImageSize(uint32_t *aImageSize) override; \
  NS_IMETHOD GetStockIcon(nsACString& aStockIcon) override; \
  NS_IMETHOD GetIconSize(nsACString& aIconSize) override; \
  NS_IMETHOD GetIconState(nsACString& aIconState) override; \
  NS_IMETHOD GetContentType(nsACString& aContentType) override; \
  NS_IMETHOD GetFileExtension(nsACString& aFileExtension) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIMOZICONURI \
  nsresult GetIconURL(nsIURL **aIconURL); \
  nsresult GetImageSize(uint32_t *aImageSize); \
  nsresult GetStockIcon(nsACString& aStockIcon); \
  nsresult GetIconSize(nsACString& aIconSize); \
  nsresult GetIconState(nsACString& aIconState); \
  nsresult GetContentType(nsACString& aContentType); \
  nsresult GetFileExtension(nsACString& aFileExtension); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIMOZICONURI(_to) \
  NS_IMETHOD GetIconURL(nsIURL **aIconURL) override { return _to GetIconURL(aIconURL); } \
  NS_IMETHOD GetImageSize(uint32_t *aImageSize) override { return _to GetImageSize(aImageSize); } \
  NS_IMETHOD GetStockIcon(nsACString& aStockIcon) override { return _to GetStockIcon(aStockIcon); } \
  NS_IMETHOD GetIconSize(nsACString& aIconSize) override { return _to GetIconSize(aIconSize); } \
  NS_IMETHOD GetIconState(nsACString& aIconState) override { return _to GetIconState(aIconState); } \
  NS_IMETHOD GetContentType(nsACString& aContentType) override { return _to GetContentType(aContentType); } \
  NS_IMETHOD GetFileExtension(nsACString& aFileExtension) override { return _to GetFileExtension(aFileExtension); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIMOZICONURI(_to) \
  NS_IMETHOD GetIconURL(nsIURL **aIconURL) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIconURL(aIconURL); } \
  NS_IMETHOD GetImageSize(uint32_t *aImageSize) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetImageSize(aImageSize); } \
  NS_IMETHOD GetStockIcon(nsACString& aStockIcon) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetStockIcon(aStockIcon); } \
  NS_IMETHOD GetIconSize(nsACString& aIconSize) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIconSize(aIconSize); } \
  NS_IMETHOD GetIconState(nsACString& aIconState) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIconState(aIconState); } \
  NS_IMETHOD GetContentType(nsACString& aContentType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetContentType(aContentType); } \
  NS_IMETHOD GetFileExtension(nsACString& aFileExtension) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFileExtension(aFileExtension); } 


// CID for nsMozIconURI, if implemented on this platform.
#define NS_MOZICONURI_CID                            \
{                                                    \
    0x43a88e0e,                                      \
    0x2d37,                                          \
    0x11d5,                                          \
    { 0x99, 0x7, 0x0, 0x10, 0x83, 0x1, 0xe, 0x9b }   \
}
#define NS_MOZICONURIMUTATOR_CID                     \
{                                                    \
    0x1460df3b,                                      \
    0x774c,                                          \
    0x4205,                                          \
    {0x83, 0x49, 0x83, 0x8e, 0x50, 0x7c, 0x3e, 0xf9} \
}

#endif /* __gen_nsIIconURI_h__ */
