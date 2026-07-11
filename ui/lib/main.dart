import 'package:flutter/material.dart';
import 'package:flutter_map/flutter_map.dart';
import 'package:latlong2/latlong.dart';

void main() {
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'SG Gender Distribution Heatmap',
      theme: ThemeData.dark(useMaterial3: true),
      home: const MapDashboard(),
      debugShowCheckedModeBanner: false,
    );
  }
}

class MapDashboard extends StatefulWidget {
  const MapDashboard({super.key});

  @override
  State<MapDashboard> createState() => _MapDashboardState();
}

class _MapDashboardState extends State<MapDashboard> {
  String _activeLayer = 'Combined';

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Stack(
        children: [
          // 1. Map Interface Canvas
          FlutterMap(
            options: const MapOptions(
              // Perfectly centered over Singapore's geographic coordinate center
              initialCenter: LatLng(1.3521, 103.8198),
              initialZoom: 11.5,
              maxZoom: 15,
              minZoom: 10,
            ),
            children: [
              TileLayer(
                urlTemplate: 'https://{s}.basemaps.cartocdn.com/dark_all/{z}/{x}/{y}{r}.png',
                subdomains: const ['a', 'b', 'c', 'd'],
              ),
              // Future Phase note: PolygonLayer will live right here
            ],
          ),

          // 2. Overlay Floating Control Dashboard Panel
          Positioned(
            top: 20,
            left: 20,
            child: Container(
              padding: const EdgeInsets.all(16),
              decoration: BoxDecoration(
                color: Colors.black.withOpacity(0.85),
                borderRadius: BorderRadius.circular(12),
                border: Border.all(color: Colors.white24, width: 1),
              ),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                mainAxisSize: MainAxisSize.min,
                children: [
                  const Text(
                    'Singapore Gender Distribution',
                    style: TextStyle(fontSize: 16, fontWeight: FontWeight.bold),
                  ),
                  const SizedBox(height: 12),
                  Row(
                    children: ['Men', 'Women', 'Combined'].map((layer) {
                      final isSelected = _activeLayer == layer;
                      return Padding(
                        padding: const EdgeInsets.symmetric(horizontal: 4.0),
                        child: ElevatedButton(
                          style: ElevatedButton.styleFrom(
                            backgroundColor: isSelected ? Colors.blueAccent : Colors.grey[800],
                            foregroundColor: Colors.white,
                          ),
                          onPressed: () {
                            setState(() {
                              _activeLayer = layer;
                            });
                          },
                          child: Text(layer),
                        ),
                      );
                    }).toList(),
                  ),
                ],
              ),
            ),
          ),
        ],
      ),
    );
  }
}