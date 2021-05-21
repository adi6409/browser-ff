/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/url-classifier/nsIUrlClassifierHashCompleter.idl
 */

#ifndef __gen_nsIUrlClassifierHashCompleter_h__
#define __gen_nsIUrlClassifierHashCompleter_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIArray; /* forward declaration */


/* starting interface:    nsIFullHashMatch */
#define NS_IFULLHASHMATCH_IID_STR "aabeb50e-d9f7-418e-9469-2cd9608958c0"

#define NS_IFULLHASHMATCH_IID \
  {0xaabeb50e, 0xd9f7, 0x418e, \
    { 0x94, 0x69, 0x2c, 0xd9, 0x60, 0x89, 0x58, 0xc0 }}

class NS_NO_VTABLE nsIFullHashMatch : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IFULLHASHMATCH_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIFullHashMatch;

  /* readonly attribute ACString tableName; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetTableName(nsACString& aTableName) = 0;

  /* readonly attribute ACString fullHash; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetFullHash(nsACString& aFullHash) = 0;

  /* readonly attribute uint32_t cacheDuration; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetCacheDuration(uint32_t *aCacheDuration) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIFullHashMatch, NS_IFULLHASHMATCH_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIFULLHASHMATCH \
  NS_IMETHOD GetTableName(nsACString& aTableName) override; \
  NS_IMETHOD GetFullHash(nsACString& aFullHash) override; \
  NS_IMETHOD GetCacheDuration(uint32_t *aCacheDuration) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIFULLHASHMATCH \
  nsresult GetTableName(nsACString& aTableName); \
  nsresult GetFullHash(nsACString& aFullHash); \
  nsresult GetCacheDuration(uint32_t *aCacheDuration); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIFULLHASHMATCH(_to) \
  NS_IMETHOD GetTableName(nsACString& aTableName) override { return _to GetTableName(aTableName); } \
  NS_IMETHOD GetFullHash(nsACString& aFullHash) override { return _to GetFullHash(aFullHash); } \
  NS_IMETHOD GetCacheDuration(uint32_t *aCacheDuration) override { return _to GetCacheDuration(aCacheDuration); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIFULLHASHMATCH(_to) \
  NS_IMETHOD GetTableName(nsACString& aTableName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTableName(aTableName); } \
  NS_IMETHOD GetFullHash(nsACString& aFullHash) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFullHash(aFullHash); } \
  NS_IMETHOD GetCacheDuration(uint32_t *aCacheDuration) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCacheDuration(aCacheDuration); } 


/* starting interface:    nsIUrlClassifierHashCompleterCallback */
#define NS_IURLCLASSIFIERHASHCOMPLETERCALLBACK_IID_STR "da16de40-df26-414d-bde7-c4faf4504868"

#define NS_IURLCLASSIFIERHASHCOMPLETERCALLBACK_IID \
  {0xda16de40, 0xdf26, 0x414d, \
    { 0xbd, 0xe7, 0xc4, 0xfa, 0xf4, 0x50, 0x48, 0x68 }}

class NS_NO_VTABLE nsIUrlClassifierHashCompleterCallback : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IURLCLASSIFIERHASHCOMPLETERCALLBACK_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIUrlClassifierHashCompleterCallback;

  /* void completionV2 (in ACString hash, in ACString table, in uint32_t chunkId); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD CompletionV2(const nsACString& hash, const nsACString& table, uint32_t chunkId) = 0;

  /* void completionV4 (in ACString partialHash, in ACString table, in uint32_t negativeCacheDuration, in nsIArray fullHashes); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD CompletionV4(const nsACString& partialHash, const nsACString& table, uint32_t negativeCacheDuration, nsIArray *fullHashes) = 0;

  /* void completionFinished (in nsresult status); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD CompletionFinished(nsresult status) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIUrlClassifierHashCompleterCallback, NS_IURLCLASSIFIERHASHCOMPLETERCALLBACK_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIURLCLASSIFIERHASHCOMPLETERCALLBACK \
  NS_IMETHOD CompletionV2(const nsACString& hash, const nsACString& table, uint32_t chunkId) override; \
  NS_IMETHOD CompletionV4(const nsACString& partialHash, const nsACString& table, uint32_t negativeCacheDuration, nsIArray *fullHashes) override; \
  NS_IMETHOD CompletionFinished(nsresult status) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIURLCLASSIFIERHASHCOMPLETERCALLBACK \
  nsresult CompletionV2(const nsACString& hash, const nsACString& table, uint32_t chunkId); \
  nsresult CompletionV4(const nsACString& partialHash, const nsACString& table, uint32_t negativeCacheDuration, nsIArray *fullHashes); \
  nsresult CompletionFinished(nsresult status); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIURLCLASSIFIERHASHCOMPLETERCALLBACK(_to) \
  NS_IMETHOD CompletionV2(const nsACString& hash, const nsACString& table, uint32_t chunkId) override { return _to CompletionV2(hash, table, chunkId); } \
  NS_IMETHOD CompletionV4(const nsACString& partialHash, const nsACString& table, uint32_t negativeCacheDuration, nsIArray *fullHashes) override { return _to CompletionV4(partialHash, table, negativeCacheDuration, fullHashes); } \
  NS_IMETHOD CompletionFinished(nsresult status) override { return _to CompletionFinished(status); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIURLCLASSIFIERHASHCOMPLETERCALLBACK(_to) \
  NS_IMETHOD CompletionV2(const nsACString& hash, const nsACString& table, uint32_t chunkId) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CompletionV2(hash, table, chunkId); } \
  NS_IMETHOD CompletionV4(const nsACString& partialHash, const nsACString& table, uint32_t negativeCacheDuration, nsIArray *fullHashes) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CompletionV4(partialHash, table, negativeCacheDuration, fullHashes); } \
  NS_IMETHOD CompletionFinished(nsresult status) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CompletionFinished(status); } 


/* starting interface:    nsIUrlClassifierHashCompleter */
#define NS_IURLCLASSIFIERHASHCOMPLETER_IID_STR "231fb2ad-ea8a-4e63-a331-eafc3b434811"

#define NS_IURLCLASSIFIERHASHCOMPLETER_IID \
  {0x231fb2ad, 0xea8a, 0x4e63, \
    { 0xa3, 0x31, 0xea, 0xfc, 0x3b, 0x43, 0x48, 0x11 }}

class NS_NO_VTABLE nsIUrlClassifierHashCompleter : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IURLCLASSIFIERHASHCOMPLETER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIUrlClassifierHashCompleter;

  /* void complete (in ACString partialHash, in ACString gethashUrl, in ACString tableName, in nsIUrlClassifierHashCompleterCallback callback); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Complete(const nsACString& partialHash, const nsACString& gethashUrl, const nsACString& tableName, nsIUrlClassifierHashCompleterCallback *callback) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIUrlClassifierHashCompleter, NS_IURLCLASSIFIERHASHCOMPLETER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIURLCLASSIFIERHASHCOMPLETER \
  NS_IMETHOD Complete(const nsACString& partialHash, const nsACString& gethashUrl, const nsACString& tableName, nsIUrlClassifierHashCompleterCallback *callback) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIURLCLASSIFIERHASHCOMPLETER \
  nsresult Complete(const nsACString& partialHash, const nsACString& gethashUrl, const nsACString& tableName, nsIUrlClassifierHashCompleterCallback *callback); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIURLCLASSIFIERHASHCOMPLETER(_to) \
  NS_IMETHOD Complete(const nsACString& partialHash, const nsACString& gethashUrl, const nsACString& tableName, nsIUrlClassifierHashCompleterCallback *callback) override { return _to Complete(partialHash, gethashUrl, tableName, callback); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIURLCLASSIFIERHASHCOMPLETER(_to) \
  NS_IMETHOD Complete(const nsACString& partialHash, const nsACString& gethashUrl, const nsACString& tableName, nsIUrlClassifierHashCompleterCallback *callback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Complete(partialHash, gethashUrl, tableName, callback); } 


#endif /* __gen_nsIUrlClassifierHashCompleter_h__ */
