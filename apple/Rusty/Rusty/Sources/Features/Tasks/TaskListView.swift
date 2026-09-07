//
//  TaskListView.swift
//  Rusty
//
//  Created by ivansaul on 9/3/26.
//

import Factory
import RustyCore
import SwiftUI

struct TaskListView: View {
    @Injected(\.taskViewModel) private var viewModel
    @State private var showSheet: Bool = false

    var body: some View {
        @Bindable var vm = viewModel
        VStack {
            TextField("New task...", text: $vm.textFieldValue)
                .padding()
                .overlay {
                    RoundedRectangle(cornerRadius: 15)
                        .stroke(Color.blue, lineWidth: 2)
                }

            Button("Add Task") {
                Task { await vm.create_task() }
            }
            .buttonStyle(.borderedProminent)

            List {
                switch viewModel.tasks {
                case .loading:
                    ProgressView()
                case let .error(error):
                    Text(error.localizedDescription)
                case let .data(tasks):
                    ForEach(tasks, id: \.id) { task in
                        TaskRowView(
                            task: task,
                            onToggle: { Task { await vm.toggle_completed(taskId: task.id) } },
                            onDelete: { Task { await vm.deleteTask(taskId: task.id) } }
                        )
                    }
                case .idle:
                    EmptyView()
                }
            }
            .listStyle(.plain)
        }
        .navigationTitle("Tasks")
        .toolbar {
            ToolbarItem(placement: .topBarTrailing) {
                Button {
                    showSheet.toggle()
                } label: {
                    Image(systemName: "plus.circle.dashed")
                }
            }
        }
        .sheet(isPresented: $showSheet, content: {
            TaskCreateView()
                .presentationDetents([.fraction(0.4)])
                .presentationDragIndicator(.visible)
        })
        .task {
            await viewModel.listTasks()
        }
    }
}

#Preview {
    NavigationStack {
        TaskListView()
    }
}
