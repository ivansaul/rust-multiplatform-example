//
//  SettingsViewModel.swift
//  Rusty
//

import Foundation
import RustyCore
import SwiftUI

@MainActor
@Observable
final class SettingsViewModel {
    private(set) var selectedColorScheme: ColorSchemeSelection

    private let settingsService: SettingsService

    init(settingsService: SettingsService) {
        self.settingsService = settingsService
        selectedColorScheme = settingsService.colorScheme()
    }

    func updateColorScheme(_ scheme: ColorSchemeSelection) {
        guard scheme != selectedColorScheme else { return }
        selectedColorScheme = scheme

        do {
            try settingsService.updateColorScheme(scheme: scheme)
        } catch {
            print(error)
        }
    }
}

extension SettingsViewModel {
    var colorScheme: ColorScheme? {
        selectedColorScheme.colorScheme
    }
}

extension ColorSchemeSelection {
    var colorScheme: ColorScheme? {
        switch self {
        case .light: return .light
        case .dark: return .dark
        case .system: return nil
        }
    }
}
