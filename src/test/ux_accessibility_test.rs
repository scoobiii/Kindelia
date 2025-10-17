// UX/GUI Accessibility Tests
// Responsável: @ui-dev @accessibility-phd
// Sprint: 6_UX_CLI_60+

#[test]
fn test_font_scaling_for_elder_users() {
    let app = UI::init();
    app.set_font_scale(1.5);
    assert_eq!(app.current_font_scale(), 1.5);
}

#[test]
fn test_voice_assistant_command() {
    let result = VoiceAssistant::parse_command("abrir carteira");
    assert_eq!(result, Some("open_wallet"));
}
