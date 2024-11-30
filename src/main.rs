use clap::Parser;
use std::str::FromStr;

#[derive(Debug, Clone)]
struct HexOrDec(i32);

impl FromStr for HexOrDec {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = if s.to_lowercase().starts_with("0x") {
            i32::from_str_radix(&s[2..], 16)
        } else {
            s.parse()
        };
        value.map(HexOrDec).map_err(|e| e.to_string())
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// OpCode in decimal or hex (with 0x prefix)
    #[arg(short, long)]
    opcode: HexOrDec,
}

fn main() {
    let args = Args::parse();
    println!("{}", get_packet_handler(args.opcode.0));
}

/// Missing packets
/// - 22 DELETE_CHARACTER_OTP_REQUEST
/// - 43 INITIAL_QUIZ_START
/// - 54 Unknown SendOpcode
/// - 56 MIGRATE_TO_CASH_SHOP_RESULT
/// - 60 PET_DEAD_MESSAGE
/// - 63 PARTY_RESULT
/// - 66 GUILD_REQUEST
/// - 112 INC_RATE_CHANGED
/// - 355 BATTLEFIELD_ENTER
/// - 378 TOURNAMENT_AVATAR_INFO
/// - 389 Unknown SendOpcode
/// - 394 CHANGE_MAPLE_POINT_RESULT
/// - 397 CASH_SHOP_MEMBER_SHOP_RESULT
/// - 401 Unknown SendOpcode
/// - 402 Unknown SendOpcode
/// - 403 Unknown SendOpcode
/// - 417 Unknown SendOpcode
/// - 418 GOLD_HAMMER_RESULT
/// - 419 Unknown SendOpcode
fn get_packet_handler(n_type: i32) -> &'static str {
    match n_type {
        0 => "CLogin::OnCheckPasswordResult((CLogin *)((char *)this - 8), iPacket)",
        1 => "CLogin::OnGuestIDLoginResult((CLogin *)((char *)this - 8), iPacket)",
        2 => "CLogin::OnAccountInfoResult((CLogin *)((char *)this - 8), iPacket)",
        3 => "CLogin::OnCheckUserLimitResult((CLogin *)((char *)this - 8), iPacket)",
        4 => "CLogin::OnSetAccountResult((CLogin *)((char *)this - 8), iPacket)",
        5 => "CLogin::OnConfirmEULAResult((CLogin *)((char *)this - 8), iPacket)",
        6 => "CLogin::OnCheckPinCodeResult((CLogin *)((char *)this - 8), iPacket)",
        7 => "CLogin::OnUpdatePinCodeResult((CLogin *)((char *)this - 8), iPacket)",
        8 => "CLogin::OnViewAllCharResult((CLogin *)((char *)this - 8), iPacket)",
        9 => "CLogin::OnSelectCharacterByVACResult((CLogin *)((char *)this - 8), iPacket)",
        10 => "CLogin::OnWorldInformation((CLogin *)((char *)this - 8), iPacket)",
        11 => "CLogin::OnSelectWorldResult((CLogin *)((char *)this - 8), iPacket)",
        12 => "CLogin::OnSelectCharacterResult((CLogin *)((char *)this - 8), iPacket)",
        13 => "CLogin::OnCheckDuplicatedIDResult((CLogin *)((char *)this - 8), iPacket)",
        14 => "CLogin::OnCreateNewCharacterResult((CLogin *)((char *)this - 8), iPacket)",
        15 => "CLogin::OnDeleteCharacterResult((CLogin *)((char *)this - 8), iPacket)",
        16 => "CClientSocket::OnMigrateCommand(this, iPacket)",
        17 => "CClientSocket::OnAliveReq(this, iPacket)",
        18 => "CClientSocket::OnAuthenCodeChanged(this, iPacket)",
        19 => "CClientSocket::OnAuthenMessage(this, iPacket)",
        20 => "CSecurityClient::OnPacket(TSingleton<CSecurityClient>::ms_pInstance, iPacket)",
        21 => "CLogin::OnEnableSPWResult((CLogin *)((char *)this - 8), iPacket)",
        23 => "CClientSocket::OnCheckCrcResult(this, iPacket)",
        24 => "CLogin::OnLatestConnectedWorld((CLogin *)((char *)this - 8), iPacket)",
        25 => "CLogin::OnRecommendWorldMessage((CLogin *)((char *)this - 8), iPacket)",
        26 => "CLogin::OnExtraCharInfoResult((CLogin *)((char *)this - 8), iPacket)",
        27 => "CLogin::OnCheckSPWResult((CLogin *)((char *)this - 8), iPacket)",
        28 => "CWvsContext::OnInventoryOperation(this, iPacket)",
        29 => "CWvsContext::OnInventoryGrow(this, iPacket)",
        30 => "CWvsContext::OnStatChanged(this, iPacket)",
        31 => "CWvsContext::OnTemporaryStatSet(this, iPacket)",
        32 => "CWvsContext::OnTemporaryStatReset(this, iPacket)",
        33 => "CWvsContext::OnForcedStatSet(this, iPacket)",
        34 => "CWvsContext::OnForcedStatReset(this, iPacket)",
        35 => "CWvsContext::OnChangeSkillRecordResult(this, iPacket)",
        36 => "CWvsContext::OnSkillUseResult(this, iPacket)",
        37 => "CWvsContext::OnGivePopularityResult(this, iPacket)",
        38 => "CWvsContext::OnMessage(this, iPacket)",
        39 => "CWvsContext::OnOpenFullClientDownloadLink(this, iPacket)",
        40 => "CWvsContext::OnMemoResult(this, iPacket)",
        41 => "CWvsContext::OnMapTransferResult(this, iPacket)",
        42 => "CWvsContext::OnAntiMacroResult(this, iPacket)",
        44 => "CWvsContext::OnClaimResult(this, iPacket)",
        45 => "CWvsContext::OnSetClaimSvrAvailableTime(this, iPacket)",
        46 => "CWvsContext::OnClaimSvrStatusChanged(this, iPacket)",
        47 => "CWvsContext::OnSetTamingMobInfo(this, iPacket)",
        48 => "CWvsContext::OnQuestClear(this, iPacket)",
        49 => "CWvsContext::OnEntrustedShopCheckResult(this, iPacket)",
        50 => "CWvsContext::OnSkillLearnItemResult(this, iPacket)",
        51 => "CWvsContext::OnSkillResetItemResult(this, iPacket)",
        52 => "CWvsContext::OnGatherItemResult(this, iPacket)",
        53 => "CWvsContext::OnSortItemResult(this, iPacket)",
        55 => "CWvsContext::OnSueCharacterResult(this, iPacket)",
        57 => "CWvsContext::OnTradeMoneyLimit(this, iPacket)",
        58 => "CWvsContext::OnSetGender(this, iPacket)",
        59 => "CWvsContext::OnGuildBBSPacket(this, iPacket)",
        61 => "CWvsContext::OnCharacterInfo(this, iPacket)",
        62 => "CWvsContext::OnPartyResult(this, iPacket)",
        64 => "CWvsContext::OnExpedtionResult(this, iPacket)",
        65 => "CWvsContext::OnFriendResult(this, iPacket)",
        67 => "CWvsContext::OnGuildResult(this, iPacket)",
        68 => "CWvsContext::OnAllianceResult(this, iPacket)",
        69 => "CWvsContext::OnTownPortal(this, iPacket)",
        70 => "CWvsContext::OnOpenGate(this, iPacket)",
        71 => "CWvsContext::OnBroadcastMsg(this, iPacket)",
        72 => "CWvsContext::OnIncubatorResult(this, iPacket)",
        73 => "CWvsContext::OnShopScannerResult(this, iPacket)",
        74 => "CWvsContext::OnShopLinkResult(this, iPacket)",
        75 => "CWvsContext::OnMarriageRequest(this, iPacket)",
        76 => "CWvsContext::OnMarriageResult(this, iPacket)",
        77 => "CWvsContext::OnWeddingGiftResult(this, iPacket)",
        78 => "CWvsContext::OnNotifyMarriedPartnerMapTransfer(this, iPacket)",
        79 => "CWvsContext::OnCashPetFoodResult(this, iPacket)",
        80 => "CWvsContext::OnSetWeekEventMessage(this, iPacket)",
        81 => "CWvsContext::OnSetPotionDiscountRate(this, iPacket)",
        82 => "CWvsContext::OnBridleMobCatchFail(this, iPacket)",
        83 => "CWvsContext::OnImitatedNPCResult(this, iPacket)",
        84 => "CWvsContext::OnImitatedNPCData(this, iPacket)",
        85 => "CWvsContext::OnLimitedNPCDisableInfo(this, iPacket)",
        86 => "CWvsContext::OnMonsterBookSetCard(this, iPacket)",
        87 => "CWvsContext::OnMonsterBookSetCover(this, iPacket)",
        88 => "CWvsContext::OnHourChanged(this, iPacket)",
        89 => "CWvsContext::OnMiniMapOnOff(this, iPacket)",
        90 => "CWvsContext::OnConsultAuthkeyUpdate(this, iPacket)",
        91 => "CWvsContext::OnClassCompetitionAuthkeyUpdate(this, iPacket)",
        92 => "CWvsContext::OnWebBoardAuthkeyUpdate(this, iPacket)",
        93 => "CWvsContext::OnSessionValue(this, iPacket)",
        94 => "CWvsContext::OnPartyValue(this, iPacket)",
        95 => "CWvsContext::OnFieldSetVariable(this, iPacket)",
        96 => "CWvsContext::OnBonusExpRateChanged(this, iPacket)",
        97 => "CWvsContext::OnPotionDiscountRateChanged(this, iPacket)",
        98 => "CWvsContext::OnFamilyChartResult(this, iPacket)",
        99 => "CWvsContext::OnFamilyInfoResult(this, iPacket)",
        100 => "CWvsContext::OnFamilyResult(this, iPacket)",
        101 => "CWvsContext::OnFamilyJoinRequest(this, iPacket)",
        102 => "CWvsContext::OnFamilyJoinRequestResult(this, iPacket)",
        103 => "CWvsContext::OnFamilyJoinAccepted(this, iPacket)",
        104 => "CWvsContext::OnFamilyPrivilegeList(this, iPacket)",
        105 => "CWvsContext::OnFamilyFamousPointIncResult(this, iPacket)",
        106 => "CWvsContext::OnFamilyNotifyLoginOrLogout(this, iPacket)",
        107 => "CWvsContext::OnFamilySetPrivilege(this, iPacket)",
        108 => "CWvsContext::OnFamilySummonRequest(this, iPacket)",
        109 => "CWvsContext::OnNotifyLevelUp(this, iPacket)",
        110 => "CWvsContext::OnNotifyWedding(this, iPacket)",
        111 => "CWvsContext::OnNotifyJobChange(this, iPacket)",
        113 => "CWvsContext::OnMapleTVUseRes(this, iPacket)",
        114 => "CWvsContext::OnAvatarMegaphoneRes(this, iPacket)",
        115 => "CWvsContext::OnSetAvatarMegaphone(this, iPacket)",
        116 => "CWvsContext::OnClearAvatarMegaphone(this, iPacket)",
        117 => "CWvsContext::OnCancelNameChangeResult(this, iPacket)",
        118 => "CWvsContext::OnCancelTransferWorldResult(this, iPacket)",
        119 => "CWvsContext::OnDestroyShopResult(this, iPacket)",
        120 => "CWvsContext::OnFakeGMNotice(this, iPacket)",
        121 => "CWvsContext::OnSuccessInUsegachaponBox(this, iPacket)",
        122 => "CWvsContext::OnNewYearCardRes(this, iPacket)",
        123 => "CWvsContext::OnRandomMorphRes(this, iPacket)",
        124 => "CWvsContext::OnCancelNameChangebyOther(this, iPacket)",
        125 => "CWvsContext::OnSetBuyEquipExt(this, iPacket)",
        126 => "CWvsContext::OnSetPassenserRequest(this, iPacket)",
        127 => "CWvsContext::OnScriptProgressMessage(this, iPacket)",
        128 => "CWvsContext::OnDataCRCCheckFailed(this, iPacket)",
        129 => "CWvsContext::OnCakePieEventResult(this, iPacket)",
        130 => "CWvsContext::OnUpdateGMBoard(this, iPacket)",
        131 => "CWvsContext::OnShowSlotMessage(this, iPacket)",
        132 => "CWvsContext::OnWildHunterInfo(this, iPacket)",
        133 => "CWvsContext::OnAccountMoreInfo(this, iPacket)",
        134 => "CWvsContext::OnFindFirend(this, iPacket)",
        135 => "CWvsContext::OnStageChange(this, iPacket)",
        136 => "CWvsContext::OnDragonBallBox(this, iPacket)",
        137 => "CWvsContext::OnAskWhetherUsePamsSong(this, iPacket)",
        138 => "CWvsContext::OnTransferChannel(this, iPacket)",
        139 => "CWvsContext::OnDisallowedDeliveryQuestList(this, iPacket)",
        140 => "CWvsContext::OnMacroSysDataInit(this, iPacket)",
        141..=143 => "CStage::OnPacket(this, nType, iPacket)",
        144..=146 => "CMapLoadable::OnPacket(this, nType, iPacket)",
        147 => "CField::OnTransferFieldReqIgnored((CField *)((char *)this - 8), iPacket)",
        148 => "CField::OnTransferChannelReqIgnored((CField *)((char *)this - 8), iPacket)",
        149 => "CField::OnFieldSpecificData((CField *)((char *)this - 8), iPacket)",
        150 => "CField::OnGroupMessage((CField *)((char *)this - 8), iPacket)",
        151 => "CField::OnWhisper((CField *)((char *)this - 8), iPacket)",
        152 => "CField::OnCoupleMessage((CField *)((char *)this - 8), iPacket)",
        153 => "CField::OnSummonItemInavailable((CField *)((char *)this - 8), iPacket)",
        154 => "CField::OnFieldEffect((CField *)((char *)this - 8), iPacket)",
        155 => "CField::OnFieldObstacleOnOff((CField *)((char *)this - 8), iPacket)",
        156 => "CField::OnFieldObstacleOnOffStatus((CField *)((char *)this - 8), iPacket)",
        157 => "CField::OnFieldObstacleAllReset((CField *)((char *)this - 8), iPacket)",
        158 => "CField::OnBlowWeather((CField *)((char *)this - 8), iPacket)",
        159 => "CField::OnPlayJukeBox((CField *)((char *)this - 8), iPacket)",
        160 => "CField::OnAdminResult((CField *)((char *)this - 8), iPacket)",
        161 => "CField::OnQuiz((CField *)((char *)this - 8), iPacket)",
        162 => "CField::OnDesc((CField *)((char *)this - 8), iPacket)",
        163 => "(*(void (__thiscall **)(int *, CInPacket *))(this[-1].m_bFearEffectOn + 44))(&this[-1].m_bFearEffectOn, iPacket)",
        164 => "CField_ContiMove::OnContiMove((CField_ContiMove *)((char *)this - 8), iPacket)",
        165 => "CField_ContiMove::OnContiState((CField_ContiMove *)((char *)this - 8), iPacket)",
        166 => "CField::OnSetQuestClear((CField *)((char *)this - 8), iPacket)",
        167 => "CField::OnSetQuestTime((CField *)((char *)this - 8), iPacket)",
        168 => "CField::OnWarnMessage((CField *)((char *)this - 8), iPacket)",
        169 => "CField::OnSetObjectState((CField *)((char *)this - 8), iPacket)",
        170 => "CField::OnDestroyClock((CField *)((char *)this - 8), iPacket)",
        171 => "CField_AriantArena::OnShowResult((CField_AriantArena *)((char *)this - 8), iPacket)",
        172 => "CField::OnStalkResult((CField *)((char *)this - 8), iPacket)",
        173 => "CField_Massacre::OnMassacreIncGauge((CField_Massacre *)((char *)this - 8), iPacket)",
        174 => "CField_MassacreResult::OnMassacreResult((CField_MassacreResult *)((char *)this - 8), iPacket)",
        175 => "CQuickslotKeyMappedMan::OnInit(TSingleton<CQuickslotKeyMappedMan>::ms_pInstance, iPacket)",
        176 => "CField::OnFootHoldInfo((CField *)((char *)this - 8), iPacket)",
        177 => "CField::OnRequestFootHoldInfo((CField *)((char *)this - 8), iPacket)",
        178 => "CField_KillCount::OnKillCountInfo((CField_KillCount *)((char *)this - 8), iPacket)",
        179..=277 => "CUserPool::OnPacket((CUserPool *)TSingleton<CUserPool>::ms_pInstance.m_pInterface, nType, iPacket)",
        278..=283 => "CSummonedPool::OnPacket(TSingleton<CSummonedPool>::ms_pInstance, nType, iPacket)",
        284..=310 => "CMobPool::OnPacket(TSingleton<CMobPool>::ms_pInstance, nType, iPacket)",
        311..=318 => "CNpcPool::OnPacket(TSingleton<CNpcPool>::ms_pInstance, nType, iPacket)",
        319..=321 => "CEmployeePool::OnPacket(TSingleton<CEmployeePool>::ms_pInstance, nType, iPacket)",
        322..=324 => "CDropPool::OnPacket(TSingleton<CDropPool>::ms_pInstance, nType, iPacket)",
        325..=327 => "CMessageBoxPool::OnPacket(TSingleton<CMessageBoxPool>::ms_pInstance, nType, iPacket)",
        328..=329 => "CAffectedAreaPool::OnPacket(TSingleton<CAffectedAreaPool>::ms_pInstance, nType, iPacket)",
        330..=331 => "CTownPortalPool::OnPacket(TSingleton<CTownPortalPool>::ms_pInstance, nType, iPacket)",
        332..=333 => "COpenGatePool::OnPacket(TSingleton<COpenGatePool>::ms_pInstance, nType, iPacket)",
        334..=337 => "CReactorPool::OnPacket(TSingleton<CReactorPool>::ms_pInstance, nType, iPacket)",
        338 => "CField_SnowBall::OnSnowBallState((CField_SnowBall *)((char *)this - 8), iPacket)",
        339 => "CField_SnowBall::OnSnowBallHit((CField_SnowBall *)((char *)this - 8), iPacket)",
        340 => "CField_SnowBall::OnSnowBallMsg((CField_SnowBall *)((char *)this - 8), iPacket)",
        341 => "CField_SnowBall::OnSnowBallTouch((CField_SnowBall *)((char *)this - 8), iPacket)",
        342 => "CField_Coconut::OnCoconutHit((CField_Coconut *)((char *)this - 8), iPacket)",
        343 => "CField_Coconut::OnCoconutScore((CField_Coconut *)((char *)this - 8), iPacket)",
        344 => "CField_GuildBoss::OnHealerMove((CField_GuildBoss *)((char *)this - 8), iPacket)",
        345 => "CField_GuildBoss::OnPulleyStateChange((CField_GuildBoss *)((char *)this - 8), iPacket)",
        346 => "CField_MonsterCarnival::OnEnter((CField_MonsterCarnival *)((char *)this - 8), iPacket)",
        347 => "CField_MonsterCarnival::OnPersonalCP((CField_MonsterCarnival *)((char *)this - 8), iPacket)",
        348 => "CField_MonsterCarnival::OnTeamCP((CField_MonsterCarnival *)((char *)this - 8), iPacket)",
        349 => "CField_MonsterCarnival::OnRequestResult((CField_MonsterCarnival *)((char *)this - 8), 1, iPacket)",
        350 => "CField_MonsterCarnival::OnRequestResult((CField_MonsterCarnival *)((char *)this - 8), 0, iPacket)",
        351 => "CField_MonsterCarnival::OnProcessForDeath((CField_MonsterCarnival *)((char *)this - 8), iPacket)",
        352 => "CField_MonsterCarnival::OnShowMemberOutMsg((CField_MonsterCarnival *)((char *)this - 8), iPacket)",
        353 => "CField_MonsterCarnival::OnShowGameResult((CField_MonsterCarnival *)((char *)this - 8), iPacket)",
        354 => "CField_AriantArena::OnUserScore((CField_AriantArena *)((char *)this - 8), iPacket)",
        356 => "CField_Battlefield::OnScoreUpdate((CField_Battlefield *)((char *)this - 8), iPacket)",
        357 => "CField_Battlefield::OnTeamChanged((CField_Battlefield *)((char *)this - 8), iPacket)",
        358 => "CField_Witchtower::OnScoreUpdate((CField_Witchtower *)((char *)this - 8), iPacket)",
        359 => "CField::OnHontaleTimer((CField *)((char *)this - 8), iPacket)",
        360 => "CField::OnChaosZakumTimer((CField *)((char *)this - 8), iPacket)",
        361 => "CField::OnHontailTimer((CField *)((char *)this - 8), iPacket)",
        362 => "CField::OnZakumTimer((CField *)((char *)this - 8), iPacket)",
        363 => "CScriptMan::OnPacket(TSingleton<CScriptMan>::ms_pInstance, 363, iPacket)",
        364..=365 => "CShopDlg::OnPacket(nType, iPacket)",
        366..=367 => "CAdminShopDlg::OnPacket(nType, iPacket)",
        368 => "CTrunkDlg::OnPacket(iPacket)",
        369..=370 => "CStoreBankDlg::OnPacket(nType, iPacket)",
        371 => "CRPSGameDlg::OnPacket(iPacket)",
        372 => "CUIMessenger::OnPacket(iPacket)",
        373 => "CMiniRoomBaseDlg::OnPacketBase(iPacket)",
        374 => "CField_Tournament::OnTournament((CField_Tournament *)((char *)this - 8), iPacket)",
        375 => "CField_Tournament::OnTournamentMatchTable((CField_Tournament *)((char *)this - 8), iPacket)",
        376 => "CField_Tournament::OnTournamentSetPrize((CField_Tournament *)((char *)this - 8), iPacket)",
        377 => "CField_Tournament::OnTournamentUEW((CField_Tournament *)((char *)this - 8), iPacket)",
        379 => "CField_Wedding::OnWeddingProgress((CField_Wedding *)((char *)this - 8), iPacket)",
        380 => "CField_Wedding::OnWeddingCeremonyEnd((CField_Wedding *)((char *)this - 8), iPacket)",
        381 => "CParcelDlg::OnPacket(iPacket)",
        382 => "CCashShop::OnChargeParamResult((CCashShop *)((char *)this - 8), iPacket)",
        383 => "CCashShop::OnQueryCashResult((CCashShop *)((char *)this - 8), iPacket)",
        384 => "CCashShop::OnCashItemResult((CCashShop *)((char *)this - 8), iPacket)",
        385 => "CCashShop::OnPurchaseExpChanged((CCashShop *)((char *)this - 8), iPacket)",
        386 => "CCashShop::OnGiftMateInfoResult((CCashShop *)((char *)this - 8), iPacket)",
        387 => "CCashShop::OnCheckDuplicatedIDResult((CCashShop *)((char *)this - 8), iPacket)",
        388 => "CCashShop::OnCheckNameChangePossibleResult((CCashShop *)((char *)this - 8), iPacket)",
        390 => "CCashShop::OnCheckTransferWorldPossibleResult((CCashShop *)((char *)this - 8), iPacket)",
        391 => "CCashShop::OnCashShopGachaponStampResult((CCashShop *)((char *)this - 8), iPacket)",
        392..=393 => "CCashShop::OnCashItemGachaponResult((CCashShop *)((char *)this - 8), iPacket)",
        395 => "CCashShop::OnOneADay((CCashShop *)((char *)this - 8), iPacket)",
        396 => "CCashShop::OnNoticeFreeCashItem((CCashShop *)((char *)this - 8), iPacket)",
        398..=400 => "CFuncKeyMappedMan::OnPacket(TSingleton<CFuncKeyMappedMan>::ms_pInstance, nType, iPacket)",
        404..=409 => "CMapleTVMan::OnPacket(TSingleton<CMapleTVMan>::ms_pInstance, nType, iPacket)",
        410..=412 => "CITC::OnPacket(CITC *this, int nType, CInPacket *iPacket)",
        413..=416 => "CField::OnCharacterSale((CField *)((char *)this - 8), nType, iPacket)",
        420..=423 => "CBattleRecordMan::OnPacket(TSingleton<CBattleRecordMan>::ms_pInstance, nType, iPacket)",
        424..=427 => "CField::OnItemUpgrade((CField *)((char *)this - 8), nType, iPacket)",
        428..=431 => "CField::OnVega((CField *)((char *)this - 8), nType, iPacket)",
        432 => "CWvsContext::OnLogoutGift((CWvsContext *)TSingleton<CWvsContext>::ms_pInstance._m_pStr, iPacket)",
        _ => "Unknown packet type"
    }
}
