//
//  AppearanceView.swift
//  Rusty
//
//  Created by ivansaul on 8/14/26.
//

import SwiftUI

struct AppearanceView: View {
    @Environment(AppearanceViewModel.self) private var appearanceVM

    var body: some View {
        @Bindable var appearance = appearanceVM

        List {
            Picker("Theme", selection: $appearance.theme) {
                ForEach(ThemePreference.allCases, id: \.self) { theme in
                    Label(
                        theme.title,
                        systemImage: theme.icon
                    )
                    .tag(theme)
                }
            }
            .pickerStyle(.inline)
        }
        .navigationTitle("Apariencia")
    }
}

#Preview {
    AppearanceView()
        .environment(AppearanceViewModel())
}
