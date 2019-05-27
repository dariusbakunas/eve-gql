#![allow(non_snake_case)]

table! {
    agtAgents (agentID) {
        agentID -> Integer,
        divisionID -> Nullable<Integer>,
        corporationID -> Nullable<Integer>,
        locationID -> Nullable<Integer>,
        level -> Nullable<Integer>,
        quality -> Nullable<Integer>,
        agentTypeID -> Nullable<Integer>,
        isLocator -> Nullable<Bool>,
    }
}

table! {
    agtAgentTypes (agentTypeID) {
        agentTypeID -> Integer,
        agentType -> Nullable<Varchar>,
    }
}

table! {
    agtResearchAgents (agentID, typeID) {
        agentID -> Integer,
        typeID -> Integer,
    }
}

table! {
    certCerts (certID) {
        certID -> Integer,
        description -> Nullable<Text>,
        groupID -> Nullable<Integer>,
        name -> Nullable<Varchar>,
    }
}

table! {
    chrAncestries (ancestryID) {
        ancestryID -> Integer,
        ancestryName -> Nullable<Varchar>,
        bloodlineID -> Nullable<Integer>,
        description -> Nullable<Varchar>,
        perception -> Nullable<Integer>,
        willpower -> Nullable<Integer>,
        charisma -> Nullable<Integer>,
        memory -> Nullable<Integer>,
        intelligence -> Nullable<Integer>,
        iconID -> Nullable<Integer>,
        shortDescription -> Nullable<Varchar>,
    }
}

table! {
    chrAttributes (attributeID) {
        attributeID -> Integer,
        attributeName -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        iconID -> Nullable<Integer>,
        shortDescription -> Nullable<Varchar>,
        notes -> Nullable<Varchar>,
    }
}

table! {
    chrBloodlines (bloodlineID) {
        bloodlineID -> Integer,
        bloodlineName -> Nullable<Varchar>,
        raceID -> Nullable<Integer>,
        description -> Nullable<Varchar>,
        maleDescription -> Nullable<Varchar>,
        femaleDescription -> Nullable<Varchar>,
        shipTypeID -> Nullable<Integer>,
        corporationID -> Nullable<Integer>,
        perception -> Nullable<Integer>,
        willpower -> Nullable<Integer>,
        charisma -> Nullable<Integer>,
        memory -> Nullable<Integer>,
        intelligence -> Nullable<Integer>,
        iconID -> Nullable<Integer>,
        shortDescription -> Nullable<Varchar>,
        shortMaleDescription -> Nullable<Varchar>,
        shortFemaleDescription -> Nullable<Varchar>,
    }
}

table! {
    chrFactions (factionID) {
        factionID -> Integer,
        factionName -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        raceIDs -> Nullable<Integer>,
        solarSystemID -> Nullable<Integer>,
        corporationID -> Nullable<Integer>,
        sizeFactor -> Nullable<Float>,
        stationCount -> Nullable<Integer>,
        stationSystemCount -> Nullable<Integer>,
        militiaCorporationID -> Nullable<Integer>,
        iconID -> Nullable<Integer>,
    }
}

table! {
    chrRaces (raceID) {
        raceID -> Integer,
        raceName -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        iconID -> Nullable<Integer>,
        shortDescription -> Nullable<Varchar>,
    }
}

table! {
    crpActivities (activityID) {
        activityID -> Integer,
        activityName -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
    }
}

table! {
    crpNPCCorporationDivisions (corporationID, divisionID) {
        corporationID -> Integer,
        divisionID -> Integer,
        size -> Nullable<Integer>,
    }
}

table! {
    crpNPCCorporationResearchFields (skillID, corporationID) {
        skillID -> Integer,
        corporationID -> Integer,
    }
}

table! {
    crpNPCCorporations (corporationID) {
        corporationID -> Integer,
        size -> Nullable<Char>,
        extent -> Nullable<Char>,
        solarSystemID -> Nullable<Integer>,
        investorID1 -> Nullable<Integer>,
        investorShares1 -> Nullable<Integer>,
        investorID2 -> Nullable<Integer>,
        investorShares2 -> Nullable<Integer>,
        investorID3 -> Nullable<Integer>,
        investorShares3 -> Nullable<Integer>,
        investorID4 -> Nullable<Integer>,
        investorShares4 -> Nullable<Integer>,
        friendID -> Nullable<Integer>,
        enemyID -> Nullable<Integer>,
        publicShares -> Nullable<Integer>,
        initialPrice -> Nullable<Integer>,
        minSecurity -> Nullable<Float>,
        scattered -> Nullable<Bool>,
        fringe -> Nullable<Integer>,
        corridor -> Nullable<Integer>,
        hub -> Nullable<Integer>,
        border -> Nullable<Integer>,
        factionID -> Nullable<Integer>,
        sizeFactor -> Nullable<Float>,
        stationCount -> Nullable<Integer>,
        stationSystemCount -> Nullable<Integer>,
        description -> Nullable<Varchar>,
        iconID -> Nullable<Integer>,
    }
}

table! {
    crpNPCCorporationTrades (corporationID, typeID) {
        corporationID -> Integer,
        typeID -> Integer,
    }
}

table! {
    crpNPCDivisions (divisionID) {
        divisionID -> Integer,
        divisionName -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        leaderType -> Nullable<Varchar>,
    }
}

table! {
    dgmAttributeCategories (categoryID) {
        categoryID -> Integer,
        categoryName -> Nullable<Varchar>,
        categoryDescription -> Nullable<Varchar>,
    }
}

table! {
    dgmAttributeTypes (attributeID) {
        attributeID -> Integer,
        attributeName -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        iconID -> Nullable<Integer>,
        defaultValue -> Nullable<Float>,
        published -> Nullable<Bool>,
        displayName -> Nullable<Varchar>,
        unitID -> Nullable<Integer>,
        stackable -> Nullable<Bool>,
        highIsGood -> Nullable<Bool>,
        categoryID -> Nullable<Integer>,
    }
}

table! {
    dgmEffects (effectID) {
        effectID -> Integer,
        effectName -> Nullable<Varchar>,
        effectCategory -> Nullable<Integer>,
        preExpression -> Nullable<Integer>,
        postExpression -> Nullable<Integer>,
        description -> Nullable<Varchar>,
        guid -> Nullable<Varchar>,
        iconID -> Nullable<Integer>,
        isOffensive -> Nullable<Bool>,
        isAssistance -> Nullable<Bool>,
        durationAttributeID -> Nullable<Integer>,
        trackingSpeedAttributeID -> Nullable<Integer>,
        dischargeAttributeID -> Nullable<Integer>,
        rangeAttributeID -> Nullable<Integer>,
        falloffAttributeID -> Nullable<Integer>,
        disallowAutoRepeat -> Nullable<Bool>,
        published -> Nullable<Bool>,
        displayName -> Nullable<Varchar>,
        isWarpSafe -> Nullable<Bool>,
        rangeChance -> Nullable<Bool>,
        electronicChance -> Nullable<Bool>,
        propulsionChance -> Nullable<Bool>,
        distribution -> Nullable<Integer>,
        sfxName -> Nullable<Varchar>,
        npcUsageChanceAttributeID -> Nullable<Integer>,
        npcActivationChanceAttributeID -> Nullable<Integer>,
        fittingUsageChanceAttributeID -> Nullable<Integer>,
        modifierInfo -> Nullable<Text>,
    }
}

table! {
    dgmExpressions (expressionID) {
        expressionID -> Integer,
        operandID -> Nullable<Integer>,
        arg1 -> Nullable<Integer>,
        arg2 -> Nullable<Integer>,
        expressionValue -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        expressionName -> Nullable<Varchar>,
        expressionTypeID -> Nullable<Integer>,
        expressionGroupID -> Nullable<Integer>,
        expressionAttributeID -> Nullable<Integer>,
    }
}

table! {
    dgmTypeAttributes (typeID, attributeID) {
        typeID -> Integer,
        attributeID -> Integer,
        valueInt -> Nullable<Integer>,
        valueFloat -> Nullable<Float>,
    }
}

table! {
    dgmTypeEffects (typeID, effectID) {
        typeID -> Integer,
        effectID -> Integer,
        isDefault -> Nullable<Bool>,
    }
}

table! {
    eveGraphics (graphicID) {
        graphicID -> Integer,
        sofFactionName -> Nullable<Varchar>,
        graphicFile -> Nullable<Varchar>,
        sofHullName -> Nullable<Varchar>,
        sofRaceName -> Nullable<Varchar>,
        description -> Nullable<Text>,
    }
}

table! {
    eveIcons (iconID) {
        iconID -> Integer,
        iconFile -> Nullable<Varchar>,
        description -> Nullable<Text>,
    }
}

table! {
    eveUnits (unitID) {
        unitID -> Integer,
        unitName -> Nullable<Varchar>,
        displayName -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
    }
}

table! {
    industryActivity (typeID, activityID) {
        typeID -> Integer,
        activityID -> Integer,
        time -> Nullable<Integer>,
    }
}

table! {
    industryBlueprints (typeID) {
        typeID -> Integer,
        maxProductionLimit -> Nullable<Integer>,
    }
}

table! {
    invCategories (categoryID) {
        categoryID -> Integer,
        categoryName -> Nullable<Varchar>,
        iconID -> Nullable<Integer>,
        published -> Nullable<Bool>,
    }
}

table! {
    invContrabandTypes (factionID, typeID) {
        factionID -> Integer,
        typeID -> Integer,
        standingLoss -> Nullable<Double>,
        confiscateMinSec -> Nullable<Double>,
        fineByValue -> Nullable<Double>,
        attackMinSec -> Nullable<Double>,
    }
}

table! {
    invControlTowerResourcePurposes (purpose) {
        purpose -> Integer,
        purposeText -> Nullable<Varchar>,
    }
}

table! {
    invControlTowerResources (controlTowerTypeID, resourceTypeID) {
        controlTowerTypeID -> Integer,
        resourceTypeID -> Integer,
        purpose -> Nullable<Integer>,
        quantity -> Nullable<Integer>,
        minSecurityLevel -> Nullable<Double>,
        factionID -> Nullable<Integer>,
    }
}

table! {
    invFlags (flagID) {
        flagID -> Integer,
        flagName -> Nullable<Varchar>,
        flagText -> Nullable<Varchar>,
        orderID -> Nullable<Integer>,
    }
}

table! {
    invGroups (groupID) {
        groupID -> Integer,
        categoryID -> Nullable<Integer>,
        groupName -> Nullable<Varchar>,
//        iconID -> Nullable<Integer>,
//        useBasePrice -> Nullable<Bool>,
//        anchored -> Nullable<Bool>,
//        anchorable -> Nullable<Bool>,
//        fittableNonSingleton -> Nullable<Bool>,
//        published -> Nullable<Bool>,
    }
}

table! {
    invItems (itemID) {
        itemID -> Integer,
        typeID -> Integer,
        ownerID -> Integer,
        locationID -> Integer,
        flagID -> Integer,
        quantity -> Integer,
    }
}

table! {
    invMarketGroups (marketGroupID) {
        marketGroupID -> Integer,
        parentGroupID -> Nullable<Integer>,
        marketGroupName -> Nullable<Varchar>,
//        description -> Nullable<Varchar>,
//        iconID -> Nullable<Integer>,
//        hasTypes -> Nullable<Bool>,
    }
}

table! {
    invMetaGroups (metaGroupID) {
        metaGroupID -> Integer,
        metaGroupName -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        iconID -> Nullable<Integer>,
    }
}

table! {
    invMetaTypes (typeID) {
        typeID -> Integer,
        parentTypeID -> Nullable<Integer>,
        metaGroupID -> Nullable<Integer>,
    }
}

table! {
    invNames (itemID) {
        itemID -> Integer,
        itemName -> Varchar,
    }
}

table! {
    invPositions (itemID) {
        itemID -> Integer,
        x -> Float,
        y -> Float,
        z -> Float,
        yaw -> Nullable<Float>,
        pitch -> Nullable<Float>,
        roll -> Nullable<Float>,
    }
}

table! {
    invTraits (traitID) {
        traitID -> Integer,
        typeID -> Nullable<Integer>,
        skillID -> Nullable<Integer>,
        bonus -> Nullable<Float>,
        bonusText -> Nullable<Text>,
        unitID -> Nullable<Integer>,
    }
}

table! {
    invTypeMaterials (typeID, materialTypeID) {
        typeID -> Integer,
        materialTypeID -> Integer,
        quantity -> Integer,
    }
}

table! {
    invTypeReactions (reactionTypeID, input, typeID) {
        reactionTypeID -> Integer,
        input -> Bool,
        typeID -> Integer,
        quantity -> Nullable<Integer>,
    }
}

table! {
    invTypes (typeID) {
        typeID -> Integer,
        groupID -> Nullable<Integer>,
        typeName -> Nullable<Varchar>,
//        description -> Nullable<Text>,
//        mass -> Nullable<Double>,
//        volume -> Nullable<Double>,
//        capacity -> Nullable<Double>,
//        portionSize -> Nullable<Integer>,
//        raceID -> Nullable<Integer>,
//        basePrice -> Nullable<Decimal>,
//        published -> Nullable<Bool>,
//        marketGroupID -> Nullable<Integer>,
//        iconID -> Nullable<Integer>,
//        soundID -> Nullable<Integer>,
//        graphicID -> Nullable<Integer>,
    }
}

table! {
    invUniqueNames (itemID) {
        itemID -> Integer,
        itemName -> Varchar,
        groupID -> Nullable<Integer>,
    }
}

table! {
    invVolumes (typeID) {
        typeID -> Integer,
        volume -> Nullable<Integer>,
    }
}

table! {
    mapCelestialStatistics (celestialID) {
        celestialID -> Integer,
        temperature -> Nullable<Double>,
        spectralClass -> Nullable<Varchar>,
        luminosity -> Nullable<Double>,
        age -> Nullable<Double>,
        life -> Nullable<Double>,
        orbitRadius -> Nullable<Double>,
        eccentricity -> Nullable<Double>,
        massDust -> Nullable<Double>,
        massGas -> Nullable<Double>,
        fragmented -> Nullable<Bool>,
        density -> Nullable<Double>,
        surfaceGravity -> Nullable<Double>,
        escapeVelocity -> Nullable<Double>,
        orbitPeriod -> Nullable<Double>,
        rotationRate -> Nullable<Double>,
        locked -> Nullable<Bool>,
        pressure -> Nullable<Double>,
        radius -> Nullable<Double>,
        mass -> Nullable<Integer>,
    }
}

table! {
    mapConstellationJumps (fromConstellationID, toConstellationID) {
        fromRegionID -> Nullable<Integer>,
        fromConstellationID -> Integer,
        toConstellationID -> Integer,
        toRegionID -> Nullable<Integer>,
    }
}

table! {
    mapConstellations (constellationID) {
        regionID -> Nullable<Integer>,
        constellationID -> Integer,
        constellationName -> Nullable<Varchar>,
        x -> Nullable<Double>,
        y -> Nullable<Double>,
        z -> Nullable<Double>,
        xMin -> Nullable<Double>,
        xMax -> Nullable<Double>,
        yMin -> Nullable<Double>,
        yMax -> Nullable<Double>,
        zMin -> Nullable<Double>,
        zMax -> Nullable<Double>,
        factionID -> Nullable<Integer>,
        radius -> Nullable<Float>,
    }
}

table! {
    mapDenormalize (itemID) {
        itemID -> Integer,
        typeID -> Nullable<Integer>,
        groupID -> Nullable<Integer>,
        solarSystemID -> Nullable<Integer>,
        constellationID -> Nullable<Integer>,
        regionID -> Nullable<Integer>,
        orbitID -> Nullable<Integer>,
        x -> Nullable<Double>,
        y -> Nullable<Double>,
        z -> Nullable<Double>,
        radius -> Nullable<Double>,
        itemName -> Nullable<Varchar>,
        security -> Nullable<Double>,
        celestialIndex -> Nullable<Integer>,
        orbitIndex -> Nullable<Integer>,
    }
}

table! {
    mapJumps (stargateID) {
        stargateID -> Integer,
        destinationID -> Nullable<Integer>,
    }
}

table! {
    mapLandmarks (landmarkID) {
        landmarkID -> Integer,
        landmarkName -> Nullable<Varchar>,
        description -> Nullable<Text>,
        locationID -> Nullable<Integer>,
        x -> Nullable<Double>,
        y -> Nullable<Double>,
        z -> Nullable<Double>,
        iconID -> Nullable<Integer>,
    }
}

table! {
    mapLocationScenes (locationID) {
        locationID -> Integer,
        graphicID -> Nullable<Integer>,
    }
}

table! {
    mapLocationWormholeClasses (locationID) {
        locationID -> Integer,
        wormholeClassID -> Nullable<Integer>,
    }
}

table! {
    mapRegionJumps (fromRegionID, toRegionID) {
        fromRegionID -> Integer,
        toRegionID -> Integer,
    }
}

table! {
    mapRegions (regionID) {
        regionID -> Integer,
        regionName -> Nullable<Varchar>,
//        x -> Nullable<Double>,
//        y -> Nullable<Double>,
//        z -> Nullable<Double>,
//        xMin -> Nullable<Double>,
//        xMax -> Nullable<Double>,
//        yMin -> Nullable<Double>,
//        yMax -> Nullable<Double>,
//        zMin -> Nullable<Double>,
//        zMax -> Nullable<Double>,
//        factionID -> Nullable<Integer>,
//        radius -> Nullable<Float>,
    }
}

table! {
    mapSolarSystemJumps (fromSolarSystemID, toSolarSystemID) {
        fromRegionID -> Nullable<Integer>,
        fromConstellationID -> Nullable<Integer>,
        fromSolarSystemID -> Integer,
        toSolarSystemID -> Integer,
        toConstellationID -> Nullable<Integer>,
        toRegionID -> Nullable<Integer>,
    }
}

table! {
    mapSolarSystems (solarSystemID) {
        regionID -> Nullable<Integer>,
        constellationID -> Nullable<Integer>,
        solarSystemID -> Integer,
        solarSystemName -> Nullable<Varchar>,
//        x -> Nullable<Double>,
//        y -> Nullable<Double>,
//        z -> Nullable<Double>,
//        xMin -> Nullable<Double>,
//        xMax -> Nullable<Double>,
//        yMin -> Nullable<Double>,
//        yMax -> Nullable<Double>,
//        zMin -> Nullable<Double>,
//        zMax -> Nullable<Double>,
//        luminosity -> Nullable<Double>,
//        border -> Nullable<Bool>,
//        fringe -> Nullable<Bool>,
//        corridor -> Nullable<Bool>,
//        hub -> Nullable<Bool>,
//        international -> Nullable<Bool>,
//        regional -> Nullable<Bool>,
//        constellation -> Nullable<Bool>,
//        security -> Nullable<Double>,
//        factionID -> Nullable<Integer>,
//        radius -> Nullable<Double>,
//        sunTypeID -> Nullable<Integer>,
//        securityClass -> Nullable<Varchar>,
    }
}

table! {
    mapUniverse (universeID) {
        universeID -> Integer,
        universeName -> Nullable<Varchar>,
        x -> Nullable<Double>,
        y -> Nullable<Double>,
        z -> Nullable<Double>,
        xMin -> Nullable<Double>,
        xMax -> Nullable<Double>,
        yMin -> Nullable<Double>,
        yMax -> Nullable<Double>,
        zMin -> Nullable<Double>,
        zMax -> Nullable<Double>,
        radius -> Nullable<Double>,
    }
}

table! {
    planetSchematics (schematicID) {
        schematicID -> Integer,
        schematicName -> Nullable<Varchar>,
        cycleTime -> Nullable<Integer>,
    }
}

table! {
    planetSchematicsPinMap (schematicID, pinTypeID) {
        schematicID -> Integer,
        pinTypeID -> Integer,
    }
}

table! {
    planetSchematicsTypeMap (schematicID, typeID) {
        schematicID -> Integer,
        typeID -> Integer,
        quantity -> Nullable<Integer>,
        isInput -> Nullable<Bool>,
    }
}

table! {
    ramActivities (activityID) {
        activityID -> Integer,
        activityName -> Nullable<Varchar>,
        iconNo -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        published -> Nullable<Bool>,
    }
}

table! {
    ramAssemblyLineStations (stationID, assemblyLineTypeID) {
        stationID -> Integer,
        assemblyLineTypeID -> Integer,
        quantity -> Nullable<Integer>,
        stationTypeID -> Nullable<Integer>,
        ownerID -> Nullable<Integer>,
        solarSystemID -> Nullable<Integer>,
        regionID -> Nullable<Integer>,
    }
}

table! {
    ramAssemblyLineTypeDetailPerCategory (assemblyLineTypeID, categoryID) {
        assemblyLineTypeID -> Integer,
        categoryID -> Integer,
        timeMultiplier -> Nullable<Double>,
        materialMultiplier -> Nullable<Double>,
        costMultiplier -> Nullable<Double>,
    }
}

table! {
    ramAssemblyLineTypeDetailPerGroup (assemblyLineTypeID, groupID) {
        assemblyLineTypeID -> Integer,
        groupID -> Integer,
        timeMultiplier -> Nullable<Double>,
        materialMultiplier -> Nullable<Double>,
        costMultiplier -> Nullable<Double>,
    }
}

table! {
    ramAssemblyLineTypes (assemblyLineTypeID) {
        assemblyLineTypeID -> Integer,
        assemblyLineTypeName -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        baseTimeMultiplier -> Nullable<Double>,
        baseMaterialMultiplier -> Nullable<Double>,
        baseCostMultiplier -> Nullable<Double>,
        volume -> Nullable<Double>,
        activityID -> Nullable<Integer>,
        minCostPerHour -> Nullable<Double>,
    }
}

table! {
    ramInstallationTypeContents (installationTypeID, assemblyLineTypeID) {
        installationTypeID -> Integer,
        assemblyLineTypeID -> Integer,
        quantity -> Nullable<Integer>,
    }
}

table! {
    skinLicense (licenseTypeID) {
        licenseTypeID -> Integer,
        duration -> Nullable<Integer>,
        skinID -> Nullable<Integer>,
    }
}

table! {
    skinMaterials (skinMaterialID) {
        skinMaterialID -> Integer,
        displayNameID -> Nullable<Integer>,
        materialSetID -> Nullable<Integer>,
    }
}

table! {
    skins (skinID) {
        skinID -> Integer,
        internalName -> Nullable<Varchar>,
        skinMaterialID -> Nullable<Integer>,
    }
}

table! {
    staOperations (operationID) {
        activityID -> Nullable<Integer>,
        operationID -> Integer,
        operationName -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        fringe -> Nullable<Integer>,
        corridor -> Nullable<Integer>,
        hub -> Nullable<Integer>,
        border -> Nullable<Integer>,
        ratio -> Nullable<Integer>,
        caldariStationTypeID -> Nullable<Integer>,
        minmatarStationTypeID -> Nullable<Integer>,
        amarrStationTypeID -> Nullable<Integer>,
        gallenteStationTypeID -> Nullable<Integer>,
        joveStationTypeID -> Nullable<Integer>,
    }
}

table! {
    staOperationServices (operationID, serviceID) {
        operationID -> Integer,
        serviceID -> Integer,
    }
}

table! {
    staServices (serviceID) {
        serviceID -> Integer,
        serviceName -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
    }
}

table! {
    staStations (stationID) {
        stationID -> Bigint,
        security -> Nullable<Double>,
        dockingCostPerVolume -> Nullable<Double>,
        maxShipVolumeDockable -> Nullable<Double>,
        officeRentalCost -> Nullable<Integer>,
        operationID -> Nullable<Integer>,
        stationTypeID -> Nullable<Integer>,
        corporationID -> Nullable<Integer>,
        solarSystemID -> Nullable<Integer>,
        constellationID -> Nullable<Integer>,
        regionID -> Nullable<Integer>,
        stationName -> Nullable<Varchar>,
        x -> Nullable<Double>,
        y -> Nullable<Double>,
        z -> Nullable<Double>,
        reprocessingEfficiency -> Nullable<Double>,
        reprocessingStationsTake -> Nullable<Double>,
        reprocessingHangarFlag -> Nullable<Integer>,
    }
}

table! {
    staStationTypes (stationTypeID) {
        stationTypeID -> Integer,
        dockEntryX -> Nullable<Double>,
        dockEntryY -> Nullable<Double>,
        dockEntryZ -> Nullable<Double>,
        dockOrientationX -> Nullable<Double>,
        dockOrientationY -> Nullable<Double>,
        dockOrientationZ -> Nullable<Double>,
        operationID -> Nullable<Integer>,
        officeSlots -> Nullable<Integer>,
        reprocessingEfficiency -> Nullable<Double>,
        conquerable -> Nullable<Bool>,
    }
}

table! {
    translationTables (sourceTable, translatedKey) {
        sourceTable -> Varchar,
        destinationTable -> Nullable<Varchar>,
        translatedKey -> Varchar,
        tcGroupID -> Nullable<Integer>,
        tcID -> Nullable<Integer>,
    }
}

table! {
    trnTranslationColumns (tcID) {
        tcGroupID -> Nullable<Integer>,
        tcID -> Integer,
        tableName -> Varchar,
        columnName -> Varchar,
        masterID -> Nullable<Varchar>,
    }
}

table! {
    trnTranslationLanguages (numericLanguageID) {
        numericLanguageID -> Integer,
        languageID -> Nullable<Varchar>,
        languageName -> Nullable<Varchar>,
    }
}

table! {
    trnTranslations (tcID, keyID, languageID) {
        tcID -> Integer,
        keyID -> Integer,
        languageID -> Varchar,
        text -> Text,
    }
}

table! {
    warCombatZones (combatZoneID) {
        combatZoneID -> Integer,
        combatZoneName -> Nullable<Varchar>,
        factionID -> Nullable<Integer>,
        centerSystemID -> Nullable<Integer>,
        description -> Nullable<Varchar>,
    }
}

table! {
    warCombatZoneSystems (solarSystemID) {
        solarSystemID -> Integer,
        combatZoneID -> Nullable<Integer>,
    }
}

allow_tables_to_appear_in_same_query!(
    agtAgents,
    agtAgentTypes,
    agtResearchAgents,
    certCerts,
    chrAncestries,
    chrAttributes,
    chrBloodlines,
    chrFactions,
    chrRaces,
    crpActivities,
    crpNPCCorporationDivisions,
    crpNPCCorporationResearchFields,
    crpNPCCorporations,
    crpNPCCorporationTrades,
    crpNPCDivisions,
    dgmAttributeCategories,
    dgmAttributeTypes,
    dgmEffects,
    dgmExpressions,
    dgmTypeAttributes,
    dgmTypeEffects,
    eveGraphics,
    eveIcons,
    eveUnits,
    industryActivity,
    industryBlueprints,
    invCategories,
    invContrabandTypes,
    invControlTowerResourcePurposes,
    invControlTowerResources,
    invFlags,
    invGroups,
    invItems,
    invMarketGroups,
    invMetaGroups,
    invMetaTypes,
    invNames,
    invPositions,
    invTraits,
    invTypeMaterials,
    invTypeReactions,
    invTypes,
    invUniqueNames,
    invVolumes,
    mapCelestialStatistics,
    mapConstellationJumps,
    mapConstellations,
    mapDenormalize,
    mapJumps,
    mapLandmarks,
    mapLocationScenes,
    mapLocationWormholeClasses,
    mapRegionJumps,
    mapRegions,
    mapSolarSystemJumps,
    mapSolarSystems,
    mapUniverse,
    planetSchematics,
    planetSchematicsPinMap,
    planetSchematicsTypeMap,
    ramActivities,
    ramAssemblyLineStations,
    ramAssemblyLineTypeDetailPerCategory,
    ramAssemblyLineTypeDetailPerGroup,
    ramAssemblyLineTypes,
    ramInstallationTypeContents,
    skinLicense,
    skinMaterials,
    skins,
    staOperations,
    staOperationServices,
    staServices,
    staStations,
    staStationTypes,
    translationTables,
    trnTranslationColumns,
    trnTranslationLanguages,
    trnTranslations,
    warCombatZones,
    warCombatZoneSystems,
);
