use crate::{
    api::RequestParams,
    common::{Country, Object},
    error::{InternalApiResult, RazorpayResult},
    ids::AccountId,
    util::deserialize_notes,
    Razorpay,
};
use chrono::{serde::ts_seconds_option, DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum AccountType {
    Standard,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum AccountStatus {
    Created,
    Activated,
    NeedsClarification,
    UnderReview,
    Suspended,
    Rejected,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BusinessType {
    #[default]
    Proprietorship,
    Partnership,
    PrivateLimited,
    PublicLimited,
    Llp,
    Ngo,
    Trust,
    Society,
    NotYetRegistered,
    Huf,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BusinessCategory {
    FinancialServices,
    Education,
    Healthcare,
    Utilities,
    Government,
    Logistics,
    ToursAndTravel,
    Transport,
    Ecommerce,
    Food,
    #[serde(rename = "it_and_software")]
    ITAndSoftware,
    Gaming,
    MediaAndEntertainment,
    Services,
    Housing,
    NotForProfit,
    Social,
    Others,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BusinessSubCategory {
    // Financial services
    MutualFund,
    Lending,
    Cryptocurrency,
    Insurance,
    #[serde(rename = "nbfc")]
    NBFC,
    Cooperatives,
    PensionFund,
    Forex,
    Securities,
    Commodities,
    Accounting,
    FinancialAdvisor,
    Crowdfunding,
    Trading,
    Betting,
    GetRichSchemes,
    MoneysendFunding,
    WireTransferAndMoneyOrders,
    TaxPreparationServices,
    TaxPayments,
    DigitalGoods,
    #[serde(rename = "atms")]
    ATMs,

    // Education
    College,
    School,
    University,
    ProfessionalCourses,
    DistanceLearning,
    DayCare,
    Coaching,
    Elearning,
    VocationalAndTradeSchools,
    SportingClubs,
    DanceHallsStudiosAndSchools,
    CorrespondenceSchools,

    // Healthcare
    Pharmacy,
    Clinic,
    Hospital,
    Lab,
    Dietician,
    Fitness,
    HealthCoaching,
    HealthProducts,
    DrugStores,
    HealthcareMarketplace,
    Osteopaths,
    MedicalEquipmentAndSupplyStores,
    PodiatristsAndChiropodists,
    DentistsAndOrthodontists,
    HardwareStores,
    Ophthalmologists,
    OrthopedicGoodsStores,
    TestingLaboratories,
    Doctors,
    HealthPractitionersMedicalServices,

    // Ecommerce
    EcommerceMarketplace,
    Agriculture,
    Books,
    ElectronicsAndFurniture,
    Coupons,
    Rental,
    FashionAndLifestyle,
    Gifting,
    Grocery,
    BabyProducts,
    OfficeSupplies,
    Wholesale,
    ReligiousProducts,
    PetProducts,
    SportsProducts,
    ArtsAndCollectibles,
    SexualWellnessProducts,
    DropShipping,
    CryptoMachinery,
    Tobacco,
    WeaponsAndAmmunitions,
    StampsAndCoinsStores,
    OfficeEquipment,
    // Should remove this `e` to fix the typo
    //                 |
    //                  "-.
    //                     |
    AutomobilePartsAndEquipements,
    //                ^^^^^^^^^^^
    //
    // TODO: fix this typo
    //
    // There's a typo in the [docs], but it could also be that the razorpay
    // server have it too, needs more testing
    //
    // [docs]: https://razorpay.com/docs/partners/aggregators/onboarding-api/appendix/
    GardenSupplyStores,
    HouseholdApplianceStores,
    NonDurableGoods,
    PawnShops,
    ElectricalPartsAndEquipment,
    WigAndToupeeShops,
    GiftNoveltyAndSouvenirShops,
    DutyFreeStores,
    OfficeAndCommercialFurniture,
    DryGoods,
    BooksAndPublications,
    CameraAndPhotographicStores,
    RecordShops,
    MeatSupplyStores,
    LeatherGoodsAndLuggage,
    SnowmobileDealers,
    MenAndBoysClothingStores,
    PaintSupplyStores,
    AutomotiveParts,
    JewelleryAndWatchStores,
    AutoStoreHomeSupplyStores,
    TentStores,
    ShoeStoresRetail,
    PetroleumAndPetroleumProducts,
    DepartmentStores,
    AutomotiveTireStores,
    SportApparelStores,
    VarietyStores,
    ChemicalsAndAlliedProducts,
    CommercialEquipments,
    FireplacePartsAndAccessories,
    FamilyClothingStores,
    FabricAndSewingStores,
    HomeSupplyWarehouse,
    ArtSupplyStores,
    CamperRecreationalAndUtilityTrailerDealers,
    ClocksAndSilverwareStores,
    DiscountStores,
    SchoolSuppliesAndStationery,
    SecondHandStores,
    WatchAndJewelleryRepairStores,
    LiquorStores,
    BoatDealers,
    // Should remove this `e`, or add an `s` after that `e` to make it
    // plural          |
    //                  "----------.
    //                              |
    OpticiansOpticalGoodsAndEyeglasseStores,
    //                      ^^^^^^^^^
    //
    // TODO: fix this typo
    //
    // There's a typo in the [docs], but it could also be that the razorpay
    // server have it too, needs more testing
    //
    // [docs]: https://razorpay.com/docs/partners/aggregators/onboarding-api/appendix/
    WholesaleFootwearStores,
    CosmeticStores,
    HomeFurnishingStores,
    AntiqueStores,
    PlumbingAndHeatingEquipment,
    TelecommunicationEquipmentStores,
    WomenClothing,
    Florists,
    ComputerSoftwareStores,
    BuildingMaterialStores,
    CandyNutConfectioneryStores,
    GlassAndWallpaperStores,
    CommercialPhotographyAndGraphicDesignServices,
    VideoGameSupplyStores,
    FuelDealers,
    DraperyAndWindowCoveringsStores,
    HearingAidsStores,
    AutomotivePaintShops,
    DurableGoodsStores,
    UniformsAndCommercialClothingStores,
    FurShops,
    IndustrialSupplies,
    BicycleStores,
    MotorcycleShopsAndDealers,
    ChildrenAndInfantsWearStores,
    WomenAccessoryStores,
    ConstructionMaterials,
    BooksPeriodicalsAndNewspaper,
    FloorCoveringStores,
    CrystalAndGlasswareStores,
    AccessoryAndApparelStores,
    HardwareEquipmentAndSupplyStores,
    ComputerPeripheralEquipmentSoftware,
    AutomobileAndTruckDealers,
    AircraftAndFarmEquipmentDealers,
    AntiqueShopsSalesAndRepairs,
    MusicStores,
    FurnitureAndHomeFurnishingStore,

    // Services
    RepairAndCleaning,
    InteriorDesignAndArchitect,
    MoversAndPackers,
    Legal,
    EventPlanning,
    ServiceCentre,
    Consulting,
    AdAndMarketing,
    ServicesClassifieds,
    MultiLevelMarketing,
    ConstructionServices,
    ArchitecturalServices,
    CarWashes,
    MotorHomeRentals,
    StenographicAndSecretarialSupportServices,
    Chiropractors,
    AutomotiveServiceShops,
    ShoeRepairShops,
    TelecommunicationService,
    Fines,
    SecurityAgencies,
    Tailors,
    TypeSettingAndEngravingServices,
    SmallApplianceRepairShops,
    PhotographyLabs,
    DryCleaners,
    MassageParlors,
    ElectronicRepairShops,
    CleaningAndSanitationServices,
    NursingCareFacilities,
    DirectMarketing,
    Lottery,
    VeterinaryServices,
    //
    // Should add an `i` after this `f` to fix the typo
    //                           |
    // .------------------------"
    //|
    AffliatedAutoRental,
    //^^^^^^^
    //
    // TODO: fix this typo
    //
    // There's a typo in the [docs], but it could also be that the razorpay
    // server have it too, needs more testing
    //
    // [docs]: https://razorpay.com/docs/partners/aggregators/onboarding-api/appendix/
    AlimonyAndChildSupport,
    AirportFlyingFields,
    GolfCourses,
    TireRetreadingAndRepairShops,
    TelevisionCableServices,
    RecreationAndSportingCamps,
    BarberAndBeautyShops,
    AgriculturalCooperatives,
    CarpentryContractors,
    WreckingAndSalvagingServices,
    AutomobileTowingservices,
    VideoTapeRentalStores,
    MiscellaneousRepairShops,
    MotorHomesAndParts,
    HorseOrDogRacing,
    LaundryServices,
    ElectricalContractors,
    DebtMarriagePersonalCounselingService,
    AirConditioningAndRefigerationRepairShops,
    CreditReportingAgencies,
    HeatingAndPlumbingContractors,
    CarpetAndUpholsteryCleaningServices,
    SwimmingPools,
    RoofingAndMetalWorkContractors,
    InternetServiceProviders,
    RecreationalCamps,
    MasonryContractors,
    ExterminatingAndDisinfectingServices,
    AmbulanceServices,
    FuneralServicesAndCrematories,
    MetalServiceCentres,
    CopyingAndBlueprintingServices,
    FuelDispensers,
    WeldingRepair,
    MobileHomeDealers,
    ConcreteWorkContractors,
    BoatRentals,
    PersonalShoppersAndShoppingClubs,
    DoorToDoorSales,
    TravelRelatedDirectMarketing,
    LotteryAndBetting,
    BandsOrchestrasAndMiscellaneousEntertainers,
    FurnitureRepairAndRefinishing,
    Contractors,
    DirectMarketingAndSubscriptionMerchants,
    TypewriterStoresSalesServiceAndRentals,
    RecreationServices,
    DirectMarketingInsuranceServices,
    BusinessServices,
    InboundTelemarketingMerchants,
    PublicWarehousing,
    OutboundTelemarketingMerchants,
    ClothingRentalStores,
    TransportationServices,
    ElectricRazorStores,
    ServiceStations,
    PhotographicStudio,
    ProfessionalServices,

    // Housing
    Developer,
    FacilityManagement,
    #[serde(rename = "rwa")]
    RWA,
    Cowoking,
    RealestateClassifieds,
    SpaceRental,

    // Not for profit
    Charity,
    Educational,
    Religious,
    Personal,

    // Social
    MatchMaking,
    SocialNetwork,
    Messaging,
    ProfessionalNetwork,
    NeighbourhoodNetwork,
    PoliticalOrganizations,
    AutomobileAssociationsAndClubs,
    CountryAndAthleticClubs,
    AssociationsAndMembership,

    // Media and entertainment
    VideoOnDemand,
    MusicStreaming,
    Multiplex,
    ContentAndPublishing,
    Ticketing,
    News,
    VideoGameArcades,
    VideoTapeProductionAndDistribution,
    BowlingAlleys,
    BilliardAndPoolEstablishments,
    AmusementParksAndCircuses,
    TicketAgencies,

    // Gaming
    GameDeveloper,
    Esports,
    OnlineCasino,
    FantasySports,
    GamingMarketplace,

    // IT and software
    Saas,
    Paas,
    Iaas,
    ConsultingAndOutsourcing,
    WebDevelopment,
    TechnicalSupport,
    DataProcessing,

    // Food
    OnlineFoodOrdering,
    Restaurant,
    FoodCourt,
    Catering,
    Alcohol,
    RestaurantSearchAndBooking,
    DairyProducts,
    Bakeries,

    // Utilities
    Electricity,
    Gas,
    Telecom,
    Water,
    Cable,
    Broadband,
    #[serde(rename = "dth")]
    DTH,
    InternetProvider,
    BillAndRechargeAggregators,

    // Government
    Central,
    State,
    IntraGovernmentPurchases,
    GovernmentPostalServices,

    // Logistics
    Freight,
    Courier,
    Warehousing,
    Distribution,
    EndToEndLogistics,
    CourierServices,

    // Tours and travel
    Aviation,
    Accommodation,
    #[serde(rename = "ota")]
    OTA,
    TravelAgency,
    TouristAttractionsAndExhibits,
    Timeshares,
    AquariumsDolphinariumsAndSeaquariums,

    // Transport
    CabHailing,
    Bus,
    TrainAndMetro,
    AutomobileRentals,
    CruiseLines,
    ParkingLotsAndGarages,
    Transportation,
    BridgeAndRoadTolls,
    FreightTransport,
    TruckAndUtilityTrailerRentals,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct Address {
    pub street1: String,
    pub street2: String,
    pub city: String,
    pub state: String,
    pub postal_code: u16,
    pub country: Country,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct BusinessAddresses {
    pub operation: Option<Address>,
    pub registered: Address,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct LegalInfo {
    pub pan: Option<String>,
    pub gst: Option<String>,
    pub cin: Option<String>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct BrandInfo {
    pub color: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct BusinessProfile {
    pub category: BusinessCategory,
    pub subcategory: BusinessSubCategory,
    #[deprecated(
        since = "0.1.0",
        note = "Please use `business_model` field instead"
    )]
    pub description: String,
    pub business_model: String,
    pub addresses: BusinessAddresses,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct ContactDetails {
    pub email: String,
    pub phone: u64,
    pub policy_url: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct ContactInfo {
    pub chargeback: ContactDetails,
    pub refund: ContactDetails,
    pub support: ContactDetails,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct App {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct Apps {
    pub websites: Vec<String>,
    pub android: Vec<App>,
    pub ios: Vec<App>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct Account {
    pub id: AccountId,
    #[serde(rename = "type")]
    pub type_: AccountType,
    pub status: AccountStatus,
    pub email: String,
    pub phone: u64,
    pub legal_business_name: String,
    pub customer_facing_business_name: Option<String>,
    pub business_type: BusinessType,
    pub reference_id: Option<String>,
    pub profile: Option<BusinessProfile>,
    pub legal_info: Option<LegalInfo>,
    pub brand: Option<BrandInfo>,
    #[serde(deserialize_with = "deserialize_notes")]
    pub notes: Object,
    pub contact_name: String,
    pub contact_info: Option<ContactInfo>,
    pub apps: Option<Apps>,
    #[serde(with = "ts_seconds_option")]
    pub activated_at: Option<DateTime<Utc>>,
    pub live: bool,
    pub hold_funds: bool,
}

#[derive(Debug, Serialize, Clone, PartialEq, Eq)]
pub struct CreateAccountApp<'a> {
    pub name: &'a str,
    pub url: &'a str,
}

#[derive(Debug, Serialize, Clone, PartialEq, Eq)]
pub struct CreateAccountApps<'a> {
    pub websites: &'a [&'a str],
    pub android: &'a [CreateAccountApp<'a>],
    pub ios: &'a [CreateAccountApp<'a>],
}

#[derive(Debug, Serialize, Clone, PartialEq, Eq)]
pub struct CreateAccountContactDetails<'a> {
    pub email: &'a str,
    pub phone: u64,
    pub policy_url: &'a str,
}

#[derive(Debug, Serialize, Clone, PartialEq, Eq)]
pub struct CreateAccountContactInfo<'a> {
    pub chargeback: CreateAccountContactDetails<'a>,
    pub refund: CreateAccountContactDetails<'a>,
    pub support: CreateAccountContactDetails<'a>,
}

#[derive(Debug, Serialize, Clone, PartialEq, Eq)]
pub struct CreateAccountBrandInfo<'a> {
    pub color: &'a str,
}

#[derive(Debug, Default, Serialize, Clone, PartialEq, Eq)]
pub struct CreateAccountLegalInfo<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pan: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gst: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cin: Option<&'a str>,
}

#[derive(Debug, Serialize, Clone, PartialEq, Eq)]
pub struct CreateAccountAddress<'a> {
    pub street1: &'a str,
    pub street2: &'a str,
    pub city: &'a str,
    pub state: &'a str,
    pub postal_code: u16,
    pub country: Country,
}

#[derive(Debug, Serialize, Clone, PartialEq, Eq)]
pub struct CreateAccountAddresses<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<CreateAccountAddress<'a>>,
    pub registered: CreateAccountAddress<'a>,
}

#[derive(Debug, Serialize, Clone, PartialEq, Eq)]
pub struct CreateAccountProfile<'a> {
    pub category: BusinessCategory,
    pub subcategory: BusinessSubCategory,
    pub business_model: &'a str,
    pub addresses: CreateAccountAddresses<'a>,
}

#[derive(Debug, Default, Serialize, Clone, PartialEq, Eq)]
pub struct CreateAccount<'a> {
    pub email: &'a str,
    pub phone: u64,
    pub legal_business_name: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_facing_business_name: Option<&'a str>,
    pub business_type: BusinessType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<CreateAccountProfile<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legal_info: Option<CreateAccountLegalInfo<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand: Option<CreateAccountBrandInfo<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Object>,
    pub contact_name: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_info: Option<CreateAccountContactInfo<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apps: Option<CreateAccountApps<'a>>,
}

impl Account {
    pub async fn create(
        razorpay: &Razorpay,
        params: CreateAccount<'_>,
    ) -> RazorpayResult<Account> {
        let res = razorpay
            .api
            .post(RequestParams {
                url: "/accounts".to_owned(),
                version: Some("v2".to_owned()),
                data: Some(params),
            })
            .await?;

        match res {
            InternalApiResult::Ok(account) => Ok(account),
            InternalApiResult::Err { error } => Err(error.into()),
        }
    }

    pub async fn fetch(
        razorpay: &Razorpay,
        account_id: &AccountId,
    ) -> RazorpayResult<Account> {
        let res = razorpay
            .api
            .get(RequestParams {
                url: format!("/accounts/{}", account_id),
                version: Some("v2".to_owned()),
                data: None::<()>,
            })
            .await?;

        match res {
            InternalApiResult::Ok(account) => Ok(account),
            InternalApiResult::Err { error } => Err(error.into()),
        }
    }

    // TODO: Add update api
    //
    // It isn't clear in the [docs] which of the field can be
    // updated, so needs more research
    //
    // [docs]: https://razorpay.com/docs/api/partners/account-onboarding/update/

    pub async fn delete(
        razorpay: &Razorpay,
        account_id: &AccountId,
    ) -> RazorpayResult<()> {
        let res: InternalApiResult<Value> = razorpay
            .api
            .delete(RequestParams {
                url: format!("/accounts/{}", account_id),
                version: Some("v2".to_owned()),
                data: None::<()>,
            })
            .await?;

        match res {
            InternalApiResult::Ok(_) => Ok(()),
            InternalApiResult::Err { error } => Err(error.into()),
        }
    }
}
