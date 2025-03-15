import React from "react";
import ExampleQuery from "../ui/ExampleQuery";

export default function ExamplesSection() {
  return (
    <div className="bg-gray-800/50 rounded-xl p-8 backdrop-blur-sm">
      <h3 className="text-2xl font-semibold mb-4 text-emerald-400">
        Example Use Cases
      </h3>
      <p className="text-gray-300 mb-6">
        Here are some useful examples to try with Yappus:
      </p>

      <div className="space-y-4">
        <ExampleQuery
          query="How do I find large files in Linux?"
          response="find / -type f -size +100M -exec ls -lh {} \; | sort -rh"
        />
        <ExampleQuery
          query="Write a bash script to backup my home directory"
          response={`#!/bin/bash\n\nBACKUP_DIR="/backup/\$(date +%Y-%m-%d)"\nmkdir -p \$BACKUP_DIR\ntar -czf \$BACKUP_DIR/home_backup.tar.gz /home/username\necho "Backup completed: \$BACKUP_DIR/home_backup.tar.gz"`}
        />
        <ExampleQuery
          query="Explain the difference between tar and zip"
          response="TAR: Archive tool that bundles files without compression by default. Add compression with flags like -z (gzip). Good for preserving Unix permissions.\n\nZIP: Both archives and compresses files. Better cross-platform compatibility with Windows. Allows adding/extracting single files without processing the whole archive."
        />
      </div>
    </div>
  );
}
