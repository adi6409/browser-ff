/* -*- Mode: C++; tab-width: 8; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/* vim: set ts=8 sts=2 et sw=2 tw=80: */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#ifndef mozilla_Components_h
#define mozilla_Components_h

#include "nsCOMPtr.h"

struct nsID;

#define NS_IMPL_COMPONENT_FACTORY(iface) \
  template <>                            \
  already_AddRefed<nsISupports> mozCreateComponent<iface>()

template <typename T>
already_AddRefed<nsISupports> mozCreateComponent();

namespace mozilla {
namespace xpcom {

enum class ModuleID : uint16_t {
  Anonymous166,
  Anonymous157,
  Anonymous115,
  Anonymous036,
  Anonymous039,
  Anonymous046,
  Anonymous159,
  Anonymous093,
  Anonymous360,
  Anonymous278,
  Anonymous108,
  Anonymous195,
  Anonymous221,
  Anonymous192,
  Anonymous103,
  Anonymous070,
  Anonymous284,
  Anonymous273,
  Anonymous047,
  Anonymous320,
  Anonymous289,
  Anonymous029,
  Anonymous315,
  Anonymous262,
  Anonymous419,
  IO,
  Anonymous059,
  Anonymous347,
  Anonymous074,
  OfflineCacheUpdate,
  Anonymous408,
  Anonymous048,
  Anonymous294,
  URILoader,
  Anonymous172,
  Anonymous438,
  Anonymous322,
  Anonymous006,
  Anonymous163,
  Anonymous254,
  Anonymous199,
  Anonymous359,
  Anonymous052,
  Anonymous067,
  Anonymous402,
  Anonymous217,
  Anonymous014,
  Anonymous165,
  Anonymous173,
  Anonymous131,
  Anonymous343,
  Anonymous445,
  Anonymous118,
  Anonymous319,
  Anonymous183,
  Anonymous337,
  Anonymous434,
  Anonymous325,
  Anonymous096,
  Anonymous042,
  Anonymous466,
  Anonymous317,
  Anonymous293,
  ThirdPartyUtil,
  Anonymous239,
  Anonymous051,
  Anonymous229,
  Anonymous019,
  SocketTransport,
  Anonymous309,
  Directory,
  Anonymous263,
  Anonymous298,
  Anonymous206,
  Anonymous060,
  Anonymous233,
  Anonymous458,
  UrlClassifierDB,
  Anonymous149,
  Anonymous100,
  GfxInfo,
  Anonymous367,
  Anonymous350,
  Anonymous440,
  Anonymous456,
  Anonymous227,
  Anonymous474,
  Anonymous225,
  Anonymous448,
  Anonymous439,
  UrlClassifierUtils,
  HttpActivityDistributor,
  Anonymous174,
  Anonymous084,
  Anonymous255,
  Anonymous427,
  Anonymous316,
  Anonymous158,
  Anonymous175,
  Anonymous249,
  Anonymous076,
  Anonymous420,
  Anonymous423,
  Anonymous424,
  Anonymous079,
  Anonymous197,
  Anonymous069,
  Anonymous235,
  Anonymous021,
  Anonymous431,
  Anonymous390,
  Anonymous446,
  Anonymous128,
  Anonymous346,
  Anonymous011,
  Anonymous247,
  Anonymous271,
  Anonymous388,
  Anonymous381,
  Anonymous429,
  Anonymous156,
  Anonymous361,
  Anonymous335,
  Anonymous303,
  Anonymous230,
  Anonymous295,
  Anonymous187,
  Anonymous058,
  Anonymous265,
  Anonymous003,
  Anonymous224,
  Anonymous362,
  Anonymous198,
  Anonymous141,
  Anonymous399,
  Anonymous413,
  Anonymous400,
  Anonymous441,
  Anonymous208,
  Anonymous380,
  Anonymous391,
  Anonymous442,
  Anonymous234,
  Anonymous160,
  FindService,
  Anonymous087,
  Anonymous308,
  Anonymous430,
  Anonymous216,
  Anonymous184,
  Anonymous032,
  Anonymous193,
  Anonymous007,
  Anonymous196,
  Anonymous130,
  Anonymous267,
  Anonymous138,
  Anonymous016,
  Anonymous150,
  Anonymous035,
  Anonymous080,
  Anonymous382,
  Anonymous387,
  Anonymous472,
  Anonymous463,
  Anonymous432,
  Anonymous389,
  Anonymous342,
  Anonymous453,
  Anonymous145,
  Anonymous407,
  Anonymous005,
  Anonymous135,
  Anonymous212,
  Anonymous300,
  Anonymous218,
  Anonymous105,
  Anonymous313,
  Anonymous245,
  Anonymous241,
  Anonymous272,
  Anonymous290,
  Anonymous114,
  DBusHandlerApp,
  Anonymous164,
  Anonymous179,
  Anonymous378,
  Anonymous375,
  Anonymous062,
  PermissionManager,
  Anonymous425,
  Anonymous081,
  Anonymous452,
  Anonymous190,
  Anonymous246,
  Anonymous372,
  Anonymous371,
  Anonymous134,
  Anonymous222,
  Anonymous038,
  Anonymous001,
  Anonymous220,
  Anonymous215,
  Anonymous082,
  Anonymous330,
  Anonymous012,
  Anonymous066,
  Anonymous143,
  Anonymous405,
  Anonymous306,
  Anonymous170,
  Anonymous403,
  Anonymous203,
  Anonymous089,
  Anonymous015,
  Anonymous443,
  Anonymous307,
  Anonymous127,
  Anonymous204,
  Anonymous162,
  Anonymous461,
  Anonymous404,
  Anonymous055,
  Anonymous194,
  Anonymous465,
  Anonymous110,
  Anonymous075,
  Anonymous094,
  Anonymous283,
  Anonymous349,
  Anonymous040,
  Anonymous147,
  Anonymous219,
  Anonymous459,
  Anonymous044,
  Anonymous034,
  Anonymous228,
  Anonymous451,
  Anonymous287,
  Anonymous333,
  Anonymous411,
  Anonymous435,
  Anonymous117,
  Anonymous207,
  Anonymous177,
  Anonymous393,
  Anonymous213,
  Anonymous107,
  Anonymous365,
  LoginReputation,
  Anonymous469,
  DocLoader,
  ChromeRegistry,
  Anonymous111,
  Anonymous068,
  Anonymous395,
  Anonymous356,
  Anonymous026,
  Anonymous467,
  Anonymous154,
  Anonymous396,
  Anonymous244,
  Anonymous120,
  ApplicationReputation,
  Anonymous090,
  Anonymous237,
  Anonymous468,
  Anonymous171,
  Anonymous043,
  Anonymous357,
  Anonymous027,
  Anonymous057,
  AlertNotification,
  StringBundle,
  Anonymous181,
  Anonymous056,
  Anonymous398,
  Anonymous191,
  Anonymous258,
  Anonymous428,
  Anonymous045,
  Anonymous176,
  Anonymous236,
  Anonymous275,
  Anonymous415,
  Anonymous327,
  Anonymous447,
  Anonymous291,
  Anonymous363,
  Anonymous077,
  Anonymous240,
  Anonymous182,
  Anonymous243,
  Anonymous392,
  Anonymous188,
  Anonymous086,
  Anonymous231,
  Anonymous020,
  Anonymous010,
  Anonymous202,
  Anonymous119,
  Anonymous358,
  Anonymous139,
  Anonymous065,
  ExtensionPolicy,
  Anonymous326,
  Anonymous433,
  Anonymous232,
  Anonymous374,
  Anonymous449,
  Anonymous437,
  Anonymous248,
  Anonymous189,
  Anonymous256,
  Anonymous352,
  Anonymous410,
  Anonymous053,
  Anonymous344,
  Anonymous276,
  Anonymous186,
  Anonymous018,
  Anonymous140,
  Anonymous286,
  Anonymous180,
  Anonymous125,
  Anonymous274,
  Anonymous462,
  Anonymous205,
  Anonymous269,
  AppStartup,
  AddonContentPolicy,
  Prefetch,
  Anonymous379,
  Anonymous460,
  Anonymous161,
  Anonymous279,
  Anonymous277,
  Anonymous310,
  Anonymous223,
  Anonymous401,
  Anonymous369,
  Anonymous050,
  Anonymous144,
  Anonymous386,
  NativeFileWatcher,
  ServiceWorkerManager,
  Anonymous351,
  Anonymous355,
  Anonymous151,
  Anonymous113,
  Anonymous455,
  Anonymous009,
  Anonymous281,
  Anonymous426,
  Anonymous312,
  Anonymous280,
  Anonymous097,
  Anonymous146,
  Anonymous259,
  Anonymous214,
  Anonymous072,
  Anonymous373,
  Anonymous253,
  Anonymous288,
  Anonymous397,
  Anonymous121,
  Anonymous109,
  Anonymous155,
  Anonymous464,
  Anonymous095,
  Anonymous064,
  Anonymous092,
  Anonymous104,
  Anonymous444,
  Anonymous004,
  Anonymous013,
  Anonymous311,
  UrlClassifierStreamUpdater,
  UUIDGenerator,
  Anonymous340,
  Anonymous122,
  Anonymous257,
  Anonymous384,
  Anonymous061,
  Anonymous123,
  Anonymous063,
  Anonymous377,
  Anonymous261,
  Anonymous025,
  Anonymous054,
  Anonymous101,
  Anonymous450,
  Anonymous041,
  Anonymous250,
  Anonymous370,
  Anonymous102,
  Anonymous331,
  Anonymous028,
  Anonymous454,
  Anonymous116,
  Anonymous137,
  Anonymous152,
  Anonymous242,
  Anonymous209,
  Anonymous008,
  Anonymous091,
  Anonymous457,
  Anonymous260,
  Anonymous211,
  Anonymous292,
  Anonymous318,
  Anonymous049,
  Anonymous071,
  Anonymous024,
  Anonymous336,
  Anonymous153,
  Anonymous417,
  Anonymous226,
  Anonymous098,
  Anonymous471,
  Anonymous251,
  Anonymous328,
  Anonymous324,
  Anonymous167,
  Anonymous364,
  Anonymous031,
  Anonymous470,
  URIFixup,
  TypeAheadFind,
  Anonymous133,
  Anonymous418,
  Alerts,
  Anonymous339,
  Anonymous099,
  Anonymous314,
  Anonymous185,
  Anonymous282,
  Anonymous304,
  Anonymous354,
  Anonymous000,
  Anonymous301,
  Anonymous073,
  Anonymous414,
  Anonymous078,
  Anonymous002,
  Anonymous422,
  Anonymous088,
  Anonymous136,
  Anonymous297,
  Anonymous023,
  Anonymous341,
  Anonymous252,
  Anonymous321,
  Anonymous406,
  Anonymous264,
  Anonymous169,
  Anonymous132,
  Anonymous285,
  Anonymous473,
  Anonymous345,
  Anonymous022,
  Anonymous421,
  Anonymous366,
  AsyncShutdown,
  Anonymous302,
  UrlClassifierPrefixSet,
  Anonymous334,
  Anonymous299,
  Anonymous394,
  Anonymous305,
  Anonymous129,
  Anonymous033,
  Anonymous083,
  DownloadPlatform,
  Anonymous168,
  Anonymous148,
  Anonymous376,
  Anonymous037,
  Anonymous412,
  Anonymous329,
  Anonymous112,
  Anonymous210,
  Anonymous296,
  Anonymous416,
  Anonymous030,
  Anonymous348,
  Anonymous409,
  StreamTransport,
  Anonymous238,
  Anonymous017,
  Anonymous385,
  Anonymous200,
  CacheStorage,
  Anonymous270,
  Anonymous338,
  Anonymous353,
  Anonymous383,
  Anonymous436,
  Anonymous178,
  Anonymous201,
  Anonymous085,
  Anonymous124,
  History,
  Anonymous142,
  Anonymous323,
  Anonymous266,
  Anonymous106,
  Anonymous268,
  Anonymous126,
  Anonymous368,
  Anonymous332,

};

class MOZ_STACK_CLASS StaticModuleHelper : public nsCOMPtr_helper {
 public:
  StaticModuleHelper(ModuleID aId, nsresult* aErrorPtr)
      : mId(aId), mErrorPtr(aErrorPtr) {}

 protected:
  nsresult SetResult(nsresult aRv) const {
    if (mErrorPtr) {
      *mErrorPtr = aRv;
    }
    return aRv;
  }

  ModuleID mId;
  nsresult* mErrorPtr;
};

class MOZ_STACK_CLASS GetServiceHelper final : public StaticModuleHelper {
 public:
  using StaticModuleHelper::StaticModuleHelper;

  nsresult NS_FASTCALL operator()(const nsIID& aIID,
                                  void** aResult) const override;
};

class MOZ_STACK_CLASS CreateInstanceHelper final : public StaticModuleHelper {
 public:
  using StaticModuleHelper::StaticModuleHelper;

  nsresult NS_FASTCALL operator()(const nsIID& aIID,
                                  void** aResult) const override;
};

class Components final {
 public:
  static const nsID& GetCID(ModuleID aID);
};

}  // namespace xpcom

namespace components {

namespace AddonContentPolicy {
static inline const nsID& CID() {
  return ::mozilla::xpcom::Components::GetCID(::mozilla::xpcom::ModuleID::AddonContentPolicy);
}

static inline ::mozilla::xpcom::GetServiceHelper Service(nsresult* aRv = nullptr) {
  return {::mozilla::xpcom::ModuleID::AddonContentPolicy, aRv};
}

static inline ::mozilla::xpcom::CreateInstanceHelper Create(nsresult* aRv = nullptr) {
  return {::mozilla::xpcom::ModuleID::AddonContentPolicy, aRv};
}
}  // namespace AddonContentPolicy

namespace AlertNotification {
static inline const nsID& CID() {
  return ::mozilla::xpcom::Components::GetCID(::mozilla::xpcom::ModuleID::AlertNotification);
}

static inline ::mozilla::xpcom::GetServiceHelper Service(nsresult* aRv = nullptr) {
  return {::mozilla::xpcom::ModuleID::AlertNotification, aRv};
}

static inline ::mozilla::xpcom::CreateInstanceHelper Create(nsresult* aRv = nullptr) {
  return {::mozilla::xpcom::ModuleID::AlertNotification, aRv};
}
}  // namespace AlertNotification

namespace Alerts {
static inline const nsID& CID() {
  return ::mozilla::xpcom::Components::GetCID(::mozilla::xpcom::ModuleID::Alerts);
}

static inline ::mozilla::xpcom::GetServiceHelper Service(nsresult* aRv = nullptr) {
  return {::mozilla::xpcom::ModuleID::Alerts, aRv};
}

static inline ::mozilla::xpcom::CreateInstanceHelper Create(nsresult* aRv = nullptr) {
  return {::mozilla::xpcom::ModuleID::Alerts, aRv};
}
}  // namespace Alerts

namespace AppStartup {
static inline const nsID& CID() {
  return ::mozilla::xpcom::Components::GetCID(::mozilla::xpcom::ModuleID::AppStartup);
}

static inline ::mozilla::xpcom::GetServiceHelper Service(nsresult* aRv = nullptr) {
  return {::mozilla::xpcom::ModuleID::AppStartup, aRv};
}

static inline ::mozilla::xpcom::CreateInstanceHelper Create(nsresult* aRv = nullptr) {
  return {::mozilla::xpcom::ModuleID::AppStartup, aRv};
}
}  // namespace AppStartup

namespace ApplicationReputation {
static inline const nsID& CID() {
  return ::mozilla::xpcom::Components::GetCID(::mozilla::xpcom::ModuleID::ApplicationReputation);
}

static inline ::mozilla::xpcom::GetServiceHelper Service(nsresult* aRv = nullptr) {
  return {::mozilla::xpcom::ModuleID::ApplicationReputation, aRv};
}
}  // namespace ApplicationReputation

namespace AsyncShutdown {
static inline const nsID& CID() {
  return ::mozilla::xpcom::Components::GetCID(::mozilla::xpcom::ModuleID::AsyncShutdown);
}

static inline ::mozilla::xpcom::GetServiceHelper Service(nsresult* aRv = nullptr) {
  return {::mozilla::xpcom::ModuleID::AsyncShutdown, aRv};
}

static inline ::mozilla::xpcom::CreateInstanceHelper Create(nsresult* aRv = nullptr) {
  return {::mozilla::xpcom::ModuleID::AsyncShutdown, aRv};
}
}  // namespace AsyncShutdown

namespace CacheStorage {
static inline const nsID& CID() {
  return ::mozilla::xpcom::Components::GetCID(::mozilla::xpcom::ModuleID::CacheStorage);
}

static inline ::mozilla::xpcom::GetServiceHelper Service(nsresult* aRv = nullptr) {
  return {::mozilla::xpcom::ModuleID::CacheStorage, aRv};
}
}  // namespace CacheStorage

namespace ChromeRegistry {
static inline const nsID& CID() {
  return ::mozilla::xpcom::Components::GetCID(::mozilla::xpcom::ModuleID::ChromeRegistry);
}

static inline ::mozilla::xpcom::GetServiceHelper Service(nsresult* aRv = nullptr) {
  return {::mozilla::xpcom::ModuleID::ChromeRegistry, aRv};
}
}  // namespace ChromeRegistry

namespace DBusHandlerApp {
static inline const nsID& CID() {
  return ::mozilla::xpcom::Components::GetCID(::mozilla::xpcom::ModuleID::DBusHandlerApp);
}

static inline ::mozilla::xpcom::GetServiceHelper Service(nsresult* aRv = nullptr) {
  return {::mozilla::xpcom::ModuleID::DBusHandlerApp, aRv};
}

static inline ::mozilla::xpcom::CreateInstanceHelper Create(nsresult* aRv = nullptr) {
  return {::mozilla::xpcom::ModuleID::DBusHandlerApp, aRv};
}
}  // namespace DBusHandlerApp

namespace Directory {
static inline const nsID& CID() {
  return ::mozilla::xpcom::Components::GetCID(::mozilla::xpcom::ModuleID::Directory);
}

static inline ::mozilla::xpcom::GetServiceHelper Service(nsresult* aRv = nullptr) {
  return {::mozilla::xpcom::ModuleID::Directory, aRv};
}

static inline ::mozilla::xpcom::CreateInstanceHelper Create(nsresult* aRv = nullptr) {
  return {::mozilla::xpcom::ModuleID::Directory, aRv};
}
}  // namespace Directory

namespace DocLoader {
static inline const nsID& CID() {
  return ::mozilla::xpcom::Components::GetCID(::mozilla::xpcom::ModuleID::DocLoader);
}

static inline ::mozilla::xpcom::GetServiceHelper Service(nsresult* aRv = nullptr) {
  return {::mozilla::xpcom::ModuleID::DocLoader, aRv};
}

static inline ::mozilla::xpcom::CreateInstanceHelper Create(nsresult* aRv = nullptr) {
  return {::mozilla::xpcom::ModuleID::DocLoader, aRv};
}
}  // namespace DocLoader

namespace DownloadPlatform {
static inline const nsID& CID() {
  return ::mozilla::xpcom::Components::GetCID(::mozilla::xpcom::ModuleID::DownloadPlatform);
}

static inline ::mozilla::xpcom::GetServiceHelper Service(nsresult* aRv = nullptr) {
  return {::mozilla::xpcom::ModuleID::DownloadPlatform, aRv};
}

static inline ::mozilla::xpcom::CreateInstanceHelper Create(nsresult* aRv = nullptr) {
  return {::mozilla::xpcom::ModuleID::DownloadPlatform, aRv};
}
}  // namespace DownloadPlatform

namespace ExtensionPolicy {
static inline const nsID& CID() {
  return ::mozilla::xpcom::Components::GetCID(::mozilla::xpcom::ModuleID::ExtensionPolicy);
}

static inline ::mozilla::xpcom::GetServiceHelper Service(nsresult* aRv = nullptr) {
  return {::mozilla::xpcom::ModuleID::ExtensionPolicy, aRv};
}
}  // namespace ExtensionPolicy

namespace FindService {
static inline const nsID& CID() {
  return ::mozilla::xpcom::Components::GetCID(::mozilla::xpcom::ModuleID::FindService);
}

static inline ::mozilla::xpcom::GetServiceHelper Service(nsresult* aRv = nullptr) {
  return {::mozilla::xpcom::ModuleID::FindService, aRv};
}

static inline ::mozilla::xpcom::CreateInstanceHelper Create(nsresult* aRv = nullptr) {
  return {::mozilla::xpcom::ModuleID::FindService, aRv};
}
}  // namespace FindService

namespace GfxInfo {
static inline const nsID& CID() {
  return ::mozilla::xpcom::Components::GetCID(::mozilla::xpcom::ModuleID::GfxInfo);
}

static inline ::mozilla::xpcom::GetServiceHelper Service(nsresult* aRv = nullptr) {
  return {::mozilla::xpcom::ModuleID::GfxInfo, aRv};
}

static inline ::mozilla::xpcom::CreateInstanceHelper Create(nsresult* aRv = nullptr) {
  return {::mozilla::xpcom::ModuleID::GfxInfo, aRv};
}
}  // namespace GfxInfo

namespace History {
static inline const nsID& CID() {
  return ::mozilla::xpcom::Components::GetCID(::mozilla::xpcom::ModuleID::History);
}

static inline ::mozilla::xpcom::GetServiceHelper Service(nsresult* aRv = nullptr) {
  return {::mozilla::xpcom::ModuleID::History, aRv};
}
}  // namespace History

namespace HttpActivityDistributor {
static inline const nsID& CID() {
  return ::mozilla::xpcom::Components::GetCID(::mozilla::xpcom::ModuleID::HttpActivityDistributor);
}

static inline ::mozilla::xpcom::GetServiceHelper Service(nsresult* aRv = nullptr) {
  return {::mozilla::xpcom::ModuleID::HttpActivityDistributor, aRv};
}

static inline ::mozilla::xpcom::CreateInstanceHelper Create(nsresult* aRv = nullptr) {
  return {::mozilla::xpcom::ModuleID::HttpActivityDistributor, aRv};
}
}  // namespace HttpActivityDistributor

namespace IO {
static inline const nsID& CID() {
  return ::mozilla::xpcom::Components::GetCID(::mozilla::xpcom::ModuleID::IO);
}

static inline ::mozilla::xpcom::GetServiceHelper Service(nsresult* aRv = nullptr) {
  return {::mozilla::xpcom::ModuleID::IO, aRv};
}
}  // namespace IO

namespace LoginReputation {
static inline const nsID& CID() {
  return ::mozilla::xpcom::Components::GetCID(::mozilla::xpcom::ModuleID::LoginReputation);
}

static inline ::mozilla::xpcom::GetServiceHelper Service(nsresult* aRv = nullptr) {
  return {::mozilla::xpcom::ModuleID::LoginReputation, aRv};
}
}  // namespace LoginReputation

namespace NativeFileWatcher {
static inline const nsID& CID() {
  return ::mozilla::xpcom::Components::GetCID(::mozilla::xpcom::ModuleID::NativeFileWatcher);
}

static inline ::mozilla::xpcom::GetServiceHelper Service(nsresult* aRv = nullptr) {
  return {::mozilla::xpcom::ModuleID::NativeFileWatcher, aRv};
}

static inline ::mozilla::xpcom::CreateInstanceHelper Create(nsresult* aRv = nullptr) {
  return {::mozilla::xpcom::ModuleID::NativeFileWatcher, aRv};
}
}  // namespace NativeFileWatcher

namespace OfflineCacheUpdate {
static inline const nsID& CID() {
  return ::mozilla::xpcom::Components::GetCID(::mozilla::xpcom::ModuleID::OfflineCacheUpdate);
}

static inline ::mozilla::xpcom::GetServiceHelper Service(nsresult* aRv = nullptr) {
  return {::mozilla::xpcom::ModuleID::OfflineCacheUpdate, aRv};
}
}  // namespace OfflineCacheUpdate

namespace PermissionManager {
static inline const nsID& CID() {
  return ::mozilla::xpcom::Components::GetCID(::mozilla::xpcom::ModuleID::PermissionManager);
}

static inline ::mozilla::xpcom::GetServiceHelper Service(nsresult* aRv = nullptr) {
  return {::mozilla::xpcom::ModuleID::PermissionManager, aRv};
}
}  // namespace PermissionManager

namespace Prefetch {
static inline const nsID& CID() {
  return ::mozilla::xpcom::Components::GetCID(::mozilla::xpcom::ModuleID::Prefetch);
}

static inline ::mozilla::xpcom::GetServiceHelper Service(nsresult* aRv = nullptr) {
  return {::mozilla::xpcom::ModuleID::Prefetch, aRv};
}

static inline ::mozilla::xpcom::CreateInstanceHelper Create(nsresult* aRv = nullptr) {
  return {::mozilla::xpcom::ModuleID::Prefetch, aRv};
}
}  // namespace Prefetch

namespace ServiceWorkerManager {
static inline const nsID& CID() {
  return ::mozilla::xpcom::Components::GetCID(::mozilla::xpcom::ModuleID::ServiceWorkerManager);
}

static inline ::mozilla::xpcom::GetServiceHelper Service(nsresult* aRv = nullptr) {
  return {::mozilla::xpcom::ModuleID::ServiceWorkerManager, aRv};
}
}  // namespace ServiceWorkerManager

namespace SocketTransport {
static inline const nsID& CID() {
  return ::mozilla::xpcom::Components::GetCID(::mozilla::xpcom::ModuleID::SocketTransport);
}

static inline ::mozilla::xpcom::GetServiceHelper Service(nsresult* aRv = nullptr) {
  return {::mozilla::xpcom::ModuleID::SocketTransport, aRv};
}
}  // namespace SocketTransport

namespace StreamTransport {
static inline const nsID& CID() {
  return ::mozilla::xpcom::Components::GetCID(::mozilla::xpcom::ModuleID::StreamTransport);
}

static inline ::mozilla::xpcom::GetServiceHelper Service(nsresult* aRv = nullptr) {
  return {::mozilla::xpcom::ModuleID::StreamTransport, aRv};
}

static inline ::mozilla::xpcom::CreateInstanceHelper Create(nsresult* aRv = nullptr) {
  return {::mozilla::xpcom::ModuleID::StreamTransport, aRv};
}
}  // namespace StreamTransport

namespace StringBundle {
static inline const nsID& CID() {
  return ::mozilla::xpcom::Components::GetCID(::mozilla::xpcom::ModuleID::StringBundle);
}

static inline ::mozilla::xpcom::GetServiceHelper Service(nsresult* aRv = nullptr) {
  return {::mozilla::xpcom::ModuleID::StringBundle, aRv};
}

static inline ::mozilla::xpcom::CreateInstanceHelper Create(nsresult* aRv = nullptr) {
  return {::mozilla::xpcom::ModuleID::StringBundle, aRv};
}
}  // namespace StringBundle

namespace ThirdPartyUtil {
static inline const nsID& CID() {
  return ::mozilla::xpcom::Components::GetCID(::mozilla::xpcom::ModuleID::ThirdPartyUtil);
}

static inline ::mozilla::xpcom::GetServiceHelper Service(nsresult* aRv = nullptr) {
  return {::mozilla::xpcom::ModuleID::ThirdPartyUtil, aRv};
}

static inline ::mozilla::xpcom::CreateInstanceHelper Create(nsresult* aRv = nullptr) {
  return {::mozilla::xpcom::ModuleID::ThirdPartyUtil, aRv};
}
}  // namespace ThirdPartyUtil

namespace TypeAheadFind {
static inline const nsID& CID() {
  return ::mozilla::xpcom::Components::GetCID(::mozilla::xpcom::ModuleID::TypeAheadFind);
}

static inline ::mozilla::xpcom::GetServiceHelper Service(nsresult* aRv = nullptr) {
  return {::mozilla::xpcom::ModuleID::TypeAheadFind, aRv};
}

static inline ::mozilla::xpcom::CreateInstanceHelper Create(nsresult* aRv = nullptr) {
  return {::mozilla::xpcom::ModuleID::TypeAheadFind, aRv};
}
}  // namespace TypeAheadFind

namespace URIFixup {
static inline const nsID& CID() {
  return ::mozilla::xpcom::Components::GetCID(::mozilla::xpcom::ModuleID::URIFixup);
}

static inline ::mozilla::xpcom::GetServiceHelper Service(nsresult* aRv = nullptr) {
  return {::mozilla::xpcom::ModuleID::URIFixup, aRv};
}

static inline ::mozilla::xpcom::CreateInstanceHelper Create(nsresult* aRv = nullptr) {
  return {::mozilla::xpcom::ModuleID::URIFixup, aRv};
}
}  // namespace URIFixup

namespace URILoader {
static inline const nsID& CID() {
  return ::mozilla::xpcom::Components::GetCID(::mozilla::xpcom::ModuleID::URILoader);
}

static inline ::mozilla::xpcom::GetServiceHelper Service(nsresult* aRv = nullptr) {
  return {::mozilla::xpcom::ModuleID::URILoader, aRv};
}

static inline ::mozilla::xpcom::CreateInstanceHelper Create(nsresult* aRv = nullptr) {
  return {::mozilla::xpcom::ModuleID::URILoader, aRv};
}
}  // namespace URILoader

namespace UUIDGenerator {
static inline const nsID& CID() {
  return ::mozilla::xpcom::Components::GetCID(::mozilla::xpcom::ModuleID::UUIDGenerator);
}

static inline ::mozilla::xpcom::GetServiceHelper Service(nsresult* aRv = nullptr) {
  return {::mozilla::xpcom::ModuleID::UUIDGenerator, aRv};
}

static inline ::mozilla::xpcom::CreateInstanceHelper Create(nsresult* aRv = nullptr) {
  return {::mozilla::xpcom::ModuleID::UUIDGenerator, aRv};
}
}  // namespace UUIDGenerator

namespace UrlClassifierDB {
static inline const nsID& CID() {
  return ::mozilla::xpcom::Components::GetCID(::mozilla::xpcom::ModuleID::UrlClassifierDB);
}

static inline ::mozilla::xpcom::GetServiceHelper Service(nsresult* aRv = nullptr) {
  return {::mozilla::xpcom::ModuleID::UrlClassifierDB, aRv};
}

static inline ::mozilla::xpcom::CreateInstanceHelper Create(nsresult* aRv = nullptr) {
  return {::mozilla::xpcom::ModuleID::UrlClassifierDB, aRv};
}
}  // namespace UrlClassifierDB

namespace UrlClassifierPrefixSet {
static inline const nsID& CID() {
  return ::mozilla::xpcom::Components::GetCID(::mozilla::xpcom::ModuleID::UrlClassifierPrefixSet);
}

static inline ::mozilla::xpcom::GetServiceHelper Service(nsresult* aRv = nullptr) {
  return {::mozilla::xpcom::ModuleID::UrlClassifierPrefixSet, aRv};
}

static inline ::mozilla::xpcom::CreateInstanceHelper Create(nsresult* aRv = nullptr) {
  return {::mozilla::xpcom::ModuleID::UrlClassifierPrefixSet, aRv};
}
}  // namespace UrlClassifierPrefixSet

namespace UrlClassifierStreamUpdater {
static inline const nsID& CID() {
  return ::mozilla::xpcom::Components::GetCID(::mozilla::xpcom::ModuleID::UrlClassifierStreamUpdater);
}

static inline ::mozilla::xpcom::GetServiceHelper Service(nsresult* aRv = nullptr) {
  return {::mozilla::xpcom::ModuleID::UrlClassifierStreamUpdater, aRv};
}

static inline ::mozilla::xpcom::CreateInstanceHelper Create(nsresult* aRv = nullptr) {
  return {::mozilla::xpcom::ModuleID::UrlClassifierStreamUpdater, aRv};
}
}  // namespace UrlClassifierStreamUpdater

namespace UrlClassifierUtils {
static inline const nsID& CID() {
  return ::mozilla::xpcom::Components::GetCID(::mozilla::xpcom::ModuleID::UrlClassifierUtils);
}

static inline ::mozilla::xpcom::GetServiceHelper Service(nsresult* aRv = nullptr) {
  return {::mozilla::xpcom::ModuleID::UrlClassifierUtils, aRv};
}

static inline ::mozilla::xpcom::CreateInstanceHelper Create(nsresult* aRv = nullptr) {
  return {::mozilla::xpcom::ModuleID::UrlClassifierUtils, aRv};
}
}  // namespace UrlClassifierUtils

}  // namespace components

}  // namespace mozilla

#endif
