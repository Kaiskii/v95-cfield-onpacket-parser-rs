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

    println!("{}", smart_parse(args.opcode));
}

fn smart_parse(op_code_dec: i32) -> String {
    match op_code_dec {
        // Specific direct matches
        147..=163 | 166..=177 | 196 | 359..=362 | 368 | 371..=373 | 381 => {
            match op_code_dec {
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
                196 => "",
                359 => "CField::OnHontaleTimer((CField *)((char *)this - 8), iPacket)",
                360 => "CField::OnChaosZakumTimer((CField *)((char *)this - 8), iPacket)",
                361 => "CField::OnHontailTimer((CField *)((char *)this - 8), iPacket)",
                362 => "CField::OnZakumTimer((CField *)((char *)this - 8), iPacket)",
                368 => "CTrunkDlg::OnPacket(iPacket)",
                371 => "CRPSGameDlg::OnPacket(iPacket)",
                372 => "CUIMessenger::OnPacket(iPacket)",
                373 => "CMiniRoomBaseDlg::OnPacketBase(iPacket)",
                381 => "CParcelDlg::OnPacket(iPacket)",
                _ => "Not within 147..=163 | 166..=177 | 196 | 359..=362 | 368 | 371..=373 | 381",
            }.to_string()
        }
        // Handle op codes > 432 with nested matches
        433..=530 => match op_code_dec {
            540 => "CScriptMan::OnPacket(TSingleton<CScriptMan>::ms_pInstance, 540, iPacket)".to_string(),
            500..=530 => "CUserPool::OnPacket((CUserPool *)TSingleton<CUserPool>::ms_pInstance.m_pInterface, nType, iPacket)".to_string(),
            _ => "Not within 433..=530".to_string(),
        },
        363 => "CScriptMan::OnPacket(TSingleton<CScriptMan>::ms_pInstance, 363, iPacket)".to_string(),
        432 => "CWvsContext::OnLogoutGift((CWvsContext *)TSingleton<CWvsContext>::ms_pInstance._m_pStr, iPacket)".to_string(),
        _ => "Not within anything".to_string(),
    }
}
