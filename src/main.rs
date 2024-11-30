use clap::Parser;

/// v95 Opcode CField Opcode Parser
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// OpCode in decimal
    #[arg(short, long)]
    opcode: i32,
}

fn main() {
    let args = Args::parse();

    println!("{}", get_packet_handler(args.opcode));
}

fn get_packet_handler(n_type: i32) -> &'static str {
    match n_type {
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
        166 => "CField::OnSetQuestClear((CField *)((char *)this - 8), iPacket)",
        167 => "CField::OnSetQuestTime((CField *)((char *)this - 8), iPacket)",
        168 => "CField::OnWarnMessage((CField *)((char *)this - 8), iPacket)",
        169 => "CField::OnSetObjectState((CField *)((char *)this - 8), iPacket)",
        170 => "CField::OnDestroyClock((CField *)((char *)this - 8), iPacket)",
        172 => "CField::OnStalkResult((CField *)((char *)this - 8), iPacket)",
        175 => "CQuickslotKeyMappedMan::OnInit(TSingleton<CQuickslotKeyMappedMan>::ms_pInstance, iPacket)",
        176 => "CField::OnFootHoldInfo((CField *)((char *)this - 8), iPacket)",
        177 => "CField::OnRequestFootHoldInfo((CField *)((char *)this - 8), iPacket)",
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
        413..=416 => "CField::OnCharacterSale((CField *)((char *)this - 8), nType, iPacket)",
        420..=423 => "CBattleRecordMan::OnPacket(TSingleton<CBattleRecordMan>::ms_pInstance, nType, iPacket)",
        424..=427 => "CField::OnItemUpgrade((CField *)((char *)this - 8), nType, iPacket)",
        428..=431 => "CField::OnVega((CField *)((char *)this - 8), nType, iPacket)",
        432 => "CWvsContext::OnLogoutGift((CWvsContext *)TSingleton<CWvsContext>::ms_pInstance._m_pStr, iPacket)",
        _ => "Unknown packet type"
    }
}
