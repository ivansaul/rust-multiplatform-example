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
    var selectedColorScheme: ColorSchemeSelection {
        didSet {
            guard oldValue != selectedColorScheme else { return }
            saveColorScheme(selectedColorScheme)
        }
    }

    var colorScheme: ColorScheme? {
        selectedColorScheme.colorScheme
    }

    private let settingsService: SettingsService

    init(settingsService: SettingsService) {
        self.settingsService = settingsService
        selectedColorScheme = settingsService.colorScheme()
    }

    private func saveColorScheme(_ scheme: ColorSchemeSelection) {
        do {
            try settingsService.updateColorScheme(scheme: scheme)
        } catch {
            print(error)
        }
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
