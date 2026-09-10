//
//  AppearanceView.swift
//  Rusty
//
//  Created by ivansaul on 8/14/26.
//

import Factory
import RustyCore
import SwiftUI

struct AppearanceView: View {
    @Injected(\.settingsViewModel) private var settingsViewModel

    var body: some View {
        List {
            Picker("Theme", selection: colorShemeBinding) {
                Text("Light").tag(ColorSchemeSelection.light)
                Text("Dark").tag(ColorSchemeSelection.dark)
                Text("System").tag(ColorSchemeSelection.system)
            }
            .pickerStyle(.inline)
        }
        .navigationTitle("Appearance")
    }
}

extension AppearanceView {
    private var colorShemeBinding: Binding<ColorSchemeSelection> {
        Binding(
            get: { settingsViewModel.selectedColorScheme },
            set: { settingsViewModel.updateColorScheme($0) }
        )
    }
}

#Preview {
    AppearanceView()
}
